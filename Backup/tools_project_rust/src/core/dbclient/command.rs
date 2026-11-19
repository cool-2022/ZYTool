// 命令对象（对应 C++ CDbCommand）：@name 命名参数 SQL、执行、内存游标与按列取值。
use rust_decimal::Decimal;

use super::connection::DbConnection;
use super::parameter::ParameterCollection;
use super::table::{columns_of, decode_row, DataColumn, DataRow, DataTable};
use super::value::{DbError, DbResult, DbValue, FromDbValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    /// 原样 SQL，支持 @name 参数
    Text,
    /// 命令文本为过程名，参数按插入顺序作为 CALL 入参
    StoredProcedure,
    /// 命令文本为表名，执行 SELECT *
    TableDirect,
}

pub struct DbCommand<'a> {
    conn: &'a DbConnection,
    command_text: String,
    command_type: CommandType,
    prepared: bool,
    pub parameters: ParameterCollection,
    columns: Vec<DataColumn>,
    rows: Vec<DataRow>,
    reader_open: bool,
    cursor: Option<usize>,
}

impl<'a> DbCommand<'a> {
    pub fn new(conn: &'a DbConnection) -> Self {
        Self {
            conn,
            command_text: String::new(),
            command_type: CommandType::Text,
            prepared: false,
            parameters: ParameterCollection::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            reader_open: false,
            cursor: None,
        }
    }

    pub fn with_text(cmd_text: impl Into<String>, conn: &'a DbConnection) -> Self {
        let mut cmd = Self::new(conn);
        cmd.command_text = cmd_text.into();
        cmd
    }

    pub fn set_command_text(&mut self, cmd_text: impl Into<String>) {
        self.command_text = cmd_text.into();
        self.prepared = false;
    }

    pub fn command_text(&self) -> &str {
        &self.command_text
    }

    pub fn set_command_type(&mut self, command_type: CommandType) {
        self.command_type = command_type;
        self.prepared = false;
    }

    /// 对应 C++ Prepare：校验 SQL 与参数绑定是否完整（sqlx 服务端自动预编译）
    pub fn prepare(&mut self) -> DbResult<()> {
        self.build_sql()?;
        self.prepared = true;
        Ok(())
    }

    pub fn is_prepared(&self) -> bool {
        self.prepared
    }

    /// 执行 INSERT/UPDATE/DELETE，返回影响行数
    pub async fn execute_non_query(&mut self) -> DbResult<u64> {
        let (sql, params) = self.build_sql()?;
        self.conn.execute(&sql, &params).await
    }

    /// 查询并装载为 DataTable；start_record 为跳过行数，max_records 为 None 表示不限
    pub async fn execute_query(&mut self, start_record: usize, max_records: Option<usize>) -> DbResult<DataTable> {
        let (sql, params) = self.build_sql()?;
        let rows = self.conn.fetch_all(&sql, &params).await?;
        let mut table = DataTable::from_rows(&rows)?;
        if let Some(max) = max_records {
            table.rows.truncate(start_record.saturating_add(max));
        }
        let skip = start_record.min(table.rows.len());
        table.rows = table.rows.split_off(skip);
        Ok(table)
    }

    /// 打开结果集游标（整表拉入内存），返回行数；之后用 read() 逐行推进
    pub async fn execute_reader(&mut self) -> DbResult<usize> {
        let (sql, params) = self.build_sql()?;
        let rows = self.conn.fetch_all(&sql, &params).await?;
        self.columns = rows.first().map(columns_of).unwrap_or_default();
        self.rows = rows.iter().map(decode_row).collect::<DbResult<_>>()?;
        self.reader_open = true;
        self.cursor = None;
        Ok(self.rows.len())
    }

    /// 取首行首列并转为 Decimal；无结果或 NULL 返回 0
    pub async fn execute_scalar(&mut self) -> DbResult<Decimal> {
        let (sql, params) = self.build_sql()?;
        let rows = self.conn.fetch_all(&sql, &params).await?;
        let Some(row) = rows.first() else {
            return Ok(Decimal::ZERO);
        };
        let values = decode_row(row)?;
        match values.first() {
            None | Some(DbValue::Null) => Ok(Decimal::ZERO),
            Some(v) => v.as_decimal().ok_or_else(|| {
                DbError::msg(format!("标量值 {:?} 无法转换为 Decimal", v), "DbCommand::execute_scalar")
            }),
        }
    }

    /// 推进游标到下一行；无更多行返回 false
    pub fn read(&mut self) -> bool {
        if !self.reader_open {
            return false;
        }
        let next = self.cursor.map_or(0, |c| c + 1);
        if next < self.rows.len() {
            self.cursor = Some(next);
            true
        } else {
            self.cursor = Some(self.rows.len());
            false
        }
    }

    /// 对应 C++ CDataReader::IsEndOfQuery
    pub fn is_end_of_query(&self) -> bool {
        !self.reader_open || self.cursor.is_some_and(|c| c >= self.rows.len())
    }

    /// 关闭游标释放结果集；复用命令对象前必须调用
    pub fn close(&mut self) {
        self.columns.clear();
        self.rows.clear();
        self.reader_open = false;
        self.cursor = None;
    }

    pub fn get_field_count(&self) -> usize {
        self.columns.len()
    }

    pub fn columns(&self) -> &[DataColumn] {
        &self.columns
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name.eq_ignore_ascii_case(name))
    }

    pub fn current_row(&self) -> DbResult<&DataRow> {
        if !self.reader_open {
            return Err(DbError::msg("结果集未打开，请先调用 execute_reader", "DbCommand::current_row"));
        }
        self.cursor
            .and_then(|c| self.rows.get(c))
            .ok_or_else(|| DbError::msg("当前无有效行，请先调用 read", "DbCommand::current_row"))
    }

    pub fn get_value(&self, column: usize) -> DbResult<&DbValue> {
        let row = self.current_row()?;
        row.get(column).ok_or_else(|| {
            DbError::msg(
                format!("列索引 {} 超出范围（共 {} 列）", column, row.len()),
                "DbCommand::get_value",
            )
        })
    }

    pub fn get_value_by_name(&self, name: &str) -> DbResult<&DbValue> {
        let idx = self
            .column_index(name)
            .ok_or_else(|| DbError::msg(format!("列 {} 不存在", name), "DbCommand::get_value_by_name"))?;
        self.get_value(idx)
    }

    pub fn is_db_null(&self, column: usize) -> DbResult<bool> {
        Ok(self.get_value(column)?.is_null())
    }

    /// 泛型取值：NULL 返回 T::default()，类型不可转换报错
    pub fn get_as<T: FromDbValue + Default>(&self, column: usize) -> DbResult<T> {
        let v = self.get_value(column)?;
        if v.is_null() {
            return Ok(T::default());
        }
        T::from_db_value(v).ok_or_else(|| {
            DbError::msg(
                format!("列 {} 的值 {:?} 无法转换为 {}", column, v, std::any::type_name::<T>()),
                "DbCommand::get_as",
            )
        })
    }

    /// 对应 C++ Get(iColumn, T& v) 重载族
    pub fn get_into<T: FromDbValue + Default>(&self, column: usize, out: &mut T) -> DbResult<()> {
        *out = self.get_as(column)?;
        Ok(())
    }

    pub fn get_i8(&self, column: usize) -> DbResult<i8> {
        self.get_as(column)
    }
    pub fn get_i16(&self, column: usize) -> DbResult<i16> {
        self.get_as(column)
    }
    pub fn get_i32(&self, column: usize) -> DbResult<i32> {
        self.get_as(column)
    }
    pub fn get_i64(&self, column: usize) -> DbResult<i64> {
        self.get_as(column)
    }
    pub fn get_u8(&self, column: usize) -> DbResult<u8> {
        self.get_as(column)
    }
    pub fn get_u16(&self, column: usize) -> DbResult<u16> {
        self.get_as(column)
    }
    pub fn get_u32(&self, column: usize) -> DbResult<u32> {
        self.get_as(column)
    }
    pub fn get_u64(&self, column: usize) -> DbResult<u64> {
        self.get_as(column)
    }
    pub fn get_f32(&self, column: usize) -> DbResult<f32> {
        self.get_as(column)
    }
    pub fn get_f64(&self, column: usize) -> DbResult<f64> {
        self.get_as(column)
    }
    pub fn get_decimal(&self, column: usize) -> DbResult<Decimal> {
        self.get_as(column)
    }
    pub fn get_datetime(&self, column: usize) -> DbResult<Option<chrono::NaiveDateTime>> {
        let v = self.get_value(column)?;
        if v.is_null() {
            return Ok(None);
        }
        v.as_datetime().map(Some).ok_or_else(|| {
            DbError::msg(format!("列 {} 的值 {:?} 无法转换为日期时间", column, v), "DbCommand::get_datetime")
        })
    }
    pub fn get_string(&self, column: usize) -> DbResult<String> {
        self.get_as(column)
    }
    pub fn get_byte_array(&self, column: usize) -> DbResult<Vec<u8>> {
        self.get_as(column)
    }

    /// 对应 C++ Fetch(ITableObject2&)：把当前行追加到 table（table 无列定义时沿用结果集列）
    pub fn fetch_into(&self, table: &mut DataTable) -> DbResult<()> {
        self.fetch_into_from(table, 0)
    }

    /// 对应 C++ Fetch(ITableObject2&, nColumnPosition)：从第 column_position 列起追加
    pub fn fetch_into_from(&self, table: &mut DataTable, column_position: usize) -> DbResult<()> {
        let row = self.current_row()?;
        if column_position > row.len() {
            return Err(DbError::msg(
                format!("起始列 {} 超出范围（共 {} 列）", column_position, row.len()),
                "DbCommand::fetch_into_from",
            ));
        }
        if table.columns.is_empty() {
            table.columns = self.columns[column_position..]
                .iter()
                .enumerate()
                .map(|(i, c)| DataColumn {
                    name: c.name.clone(),
                    type_name: c.type_name.clone(),
                    ordinal: i,
                })
                .collect();
        }
        table.add_row(row[column_position..].to_vec())
    }

    /// 对应 C++ Get(CDataTable&, rowIndex)：用当前行覆盖 table 的第 row_index 行
    pub fn fetch_into_row(&self, table: &mut DataTable, row_index: usize) -> DbResult<()> {
        let row = self.current_row()?.clone();
        table.set_row(row_index, row)
    }

    fn build_sql(&self) -> DbResult<(String, Vec<DbValue>)> {
        let text = self.command_text.trim();
        if text.is_empty() {
            return Err(DbError::msg("命令文本为空", "DbCommand::build_sql"));
        }
        match self.command_type {
            CommandType::Text => translate_named_params(text, &self.parameters),
            CommandType::StoredProcedure => {
                let placeholders: Vec<String> = (1..=self.parameters.len()).map(|i| format!("${}", i)).collect();
                let values = self.parameters.iter().map(|p| p.value.clone()).collect();
                Ok((format!("CALL {}({})", text, placeholders.join(", ")), values))
            }
            CommandType::TableDirect => Ok((format!("SELECT * FROM {}", text), Vec::new())),
        }
    }
}

/// 把 `@name` 占位符按首次出现顺序改写为 `$n`，跳过单引号字符串与 `@@`；
/// 同名多次出现复用同一序号；SQL 引用了未设置的参数时报错。
fn translate_named_params(sql: &str, params: &ParameterCollection) -> DbResult<(String, Vec<DbValue>)> {
    let mut out = String::with_capacity(sql.len());
    let mut values: Vec<DbValue> = Vec::new();
    let mut order: Vec<String> = Vec::new();
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    let mut in_quote = false;

    while i < chars.len() {
        let c = chars[i];
        if in_quote {
            out.push(c);
            if c == '\'' {
                in_quote = false;
            }
            i += 1;
            continue;
        }
        match c {
            '\'' => {
                in_quote = true;
                out.push(c);
                i += 1;
            }
            '@' if i + 1 < chars.len() && chars[i + 1] == '@' => {
                out.push_str("@@");
                i += 2;
            }
            '@' if i + 1 < chars.len() && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_') => {
                let start = i + 1;
                let mut end = start;
                while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                let name: String = chars[start..end].iter().collect();
                let key = name.to_ascii_uppercase();
                let idx = match order.iter().position(|k| *k == key) {
                    Some(pos) => pos,
                    None => {
                        let value = params.get_value(&name).ok_or_else(|| {
                            DbError::msg(format!("SQL 引用了未设置的参数 @{}", name), "DbCommand::build_sql")
                        })?;
                        order.push(key);
                        values.push(value.clone());
                        order.len() - 1
                    }
                };
                out.push('$');
                out.push_str(&(idx + 1).to_string());
                i = end;
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    Ok((out, values))
}
