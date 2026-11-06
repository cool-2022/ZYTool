// 本模块为 EI 平台对接预留的 parity 结构，尚未接入路由/服务，接入前统一允许 dead_code。
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// EI 平台（EiInfo）数据交换结构的 Rust 对应，与前端 src/utils/ei/ 保持一致。
/// 字段名按 EI 的 JSON 契约使用 camelCase 序列化；
/// Block 线格式为 meta.columns（列元数据数组）+ rows（行对象数组）。

fn default_col_type() -> String {
    "string".to_string()
}

/// sys 系统信息，字段不确定，未知属性原样透传（对应前端 EITools.SysInfo）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SysInfo {
    #[serde(flatten)]
    pub attrs: Map<String, Value>,
}

/// EDInfo 扩展数据：fblk_name/func_name 定长 63，pk_name/pk_val 定长 3（运行时以 Vec 承载）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdInfo {
    pub fblk_name: Vec<String>,
    pub func_name: Vec<String>,
    pub pk_name: Vec<String>,
    pub pk_val: Vec<String>,
}

impl Default for EdInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl EdInfo {
    pub fn new() -> Self {
        Self {
            fblk_name: vec![String::new(); 63],
            func_name: vec![String::new(); 63],
            pk_name: vec![String::new(); 3],
            pk_val: vec![String::new(); 3],
        }
    }
}

/// 列元数据（对应前端 IColumnJson / EiColumn）；regex、formatter、primaryKey 等可选属性经 extra 透传。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EiColumn {
    #[serde(default)]
    pub pos: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_col_type", rename = "type")]
    pub col_type: String,
    #[serde(default)]
    pub desc_name: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl EiColumn {
    pub fn new(name: impl Into<String>, col_type: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            col_type: col_type.into(),
            desc_name: name,
            ..Self::default()
        }
    }
}

/// block 元数据（对应前端 getBlockMeta/setBlockMeta 的 IColumnJson[] 契约）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EiBlockMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_name: Option<String>,
    #[serde(default)]
    pub columns: Vec<EiColumn>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// ArrayObject 结构的数据块（对应前端 EiBlock）：rows 等价前端 data，meta.columns 等价 columns。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EiBlock {
    /// 块名为运行时属性，JSON 中由 EiInfo.blocks 的键承载，不随块序列化。
    #[serde(skip)]
    pub name: String,
    #[serde(default)]
    pub meta: EiBlockMeta,
    #[serde(default)]
    pub rows: Vec<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub ext_attr: Map<String, Value>,
}

/// 由值推断列数据类型（对应前端 inferDataType）。
fn infer_data_type(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        _ => "object",
    }
}

impl EiBlock {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// 添加一行数据（TS addRow）。
    pub fn add_row(&mut self, row: Map<String, Value>) -> &mut Self {
        self.rows.push(row);
        self
    }

    /// 批量追加数据（TS pushData）；auto_add_column=true 时自动为未知键生成列元数据，
    /// 并向既有行写入默认值 ''（与前端 addColumn 默认行为一致）。
    pub fn push_data(&mut self, rows: Vec<Map<String, Value>>, auto_add_column: bool) {
        for row in rows {
            if auto_add_column {
                let keys: Vec<(String, &'static str)> = row
                    .iter()
                    .filter(|(k, _)| !self.contains_column(k))
                    .map(|(k, v)| (k.clone(), infer_data_type(v)))
                    .collect();
                for (key, data_type) in keys {
                    self.add_column_by_name(&key, Some(Value::String(String::new())), Some(data_type));
                }
            }
            self.rows.push(row);
        }
    }

    /// 删除指定行，返回被删除的数据（TS deleteRow）。
    pub fn delete_row(&mut self, index: usize) -> Option<Map<String, Value>> {
        if index >= self.rows.len() {
            return None;
        }
        Some(self.rows.remove(index))
    }

    /// 在指定位置插入一行（TS insertRow）；index 超过行数时追加到末尾。
    pub fn insert_row(&mut self, index: usize, row: Map<String, Value>) -> &mut Self {
        let index = index.min(self.rows.len());
        self.rows.insert(index, row);
        self
    }

    pub fn get_block_meta(&self) -> &Vec<EiColumn> {
        &self.meta.columns
    }

    pub fn set_block_meta(&mut self, meta: Vec<EiColumn>) {
        self.meta.columns = meta;
    }

    pub fn get_column(&self, column_name: &str) -> Option<&EiColumn> {
        self.meta.columns.iter().find(|c| c.name == column_name)
    }

    pub fn contains_column(&self, column_name: &str) -> bool {
        self.meta.columns.iter().any(|c| c.name == column_name)
    }

    /// 添加一列（TS addColumn）；已存在同名列时忽略，default_value 写入既有行。
    pub fn add_column(&mut self, mut column: EiColumn, default_value: Option<Value>) -> &mut Self {
        if self.contains_column(&column.name) {
            return self;
        }
        column.pos = self.meta.columns.len() as i32;
        for row in &mut self.rows {
            if !row.contains_key(&column.name) {
                row.insert(column.name.clone(), default_value.clone().unwrap_or(Value::String(String::new())));
            }
        }
        self.meta.columns.push(column);
        self
    }

    /// 以字段名添加一列（TS addColumn 的字符串形式）。
    pub fn add_column_by_name(
        &mut self,
        name: &str,
        default_value: Option<Value>,
        data_type: Option<&str>,
    ) -> &mut Self {
        self.add_column(
            EiColumn::new(name, data_type.unwrap_or("string")),
            default_value,
        )
    }

    /// 添加多列（TS addColumns 的 EiColumn[] 形式）。
    pub fn add_columns(&mut self, columns: Vec<EiColumn>) -> &mut Self {
        for col in columns {
            self.add_column(col, None);
        }
        self
    }

    /// 以字段名数组添加多列（TS addColumns 的 ...string[] 形式）。
    pub fn add_column_names(&mut self, names: &[&str]) -> &mut Self {
        for name in names {
            self.add_column_by_name(name, None, None);
        }
        self
    }

    /// 删除某个字段（连同各行数据中的该键，TS removeColumn）。
    pub fn remove_column(&mut self, column_name: &str) -> &mut Self {
        self.meta.columns.retain(|c| c.name != column_name);
        for row in &mut self.rows {
            row.remove(column_name);
        }
        self
    }

    /// 重命名字段名（连同各行数据中的键，TS renameColumn）。
    pub fn rename_column(&mut self, old_column_name: &str, new_column_name: &str) -> &mut Self {
        if let Some(col) = self.meta.columns.iter_mut().find(|c| c.name == old_column_name) {
            col.name = new_column_name.to_string();
        }
        for row in &mut self.rows {
            if let Some(v) = row.remove(old_column_name) {
                row.insert(new_column_name.to_string(), v);
            }
        }
        self
    }

    /// 复制表（含数据行，TS copy）；不指定块名时为「原名 copy」。
    pub fn copy(&self, block_name: Option<&str>) -> EiBlock {
        let mut block = self.clone();
        block.name = block_name
            .map(str::to_owned)
            .unwrap_or_else(|| format!("{} copy", self.name));
        block
    }

    /// 复制表（TS clone）；exclude_rows=true 时不带数据行。
    pub fn clone_block(&self, block_name: &str, exclude_rows: bool) -> EiBlock {
        let mut block = self.copy(Some(block_name));
        if exclude_rows {
            block.rows.clear();
        }
        block
    }

    pub fn get_cell(&self, row: usize, col: &str) -> Option<&Value> {
        self.rows.get(row)?.get(col)
    }

    /// 设置单元格；行号超出现有行时自动补空行（与前端 EIInfo.set 行为一致）。
    pub fn set_cell(&mut self, row: usize, col: &str, value: Value) {
        while self.rows.len() <= row {
            self.rows.push(Map::new());
        }
        self.rows[row].insert(col.to_string(), value);
    }

    /// 通过列名数组创建（TS build 的 string[] 形式）。
    pub fn build(block_id: &str, column_names: &[&str]) -> EiBlock {
        let mut block = EiBlock::new(block_id);
        block.add_column_names(column_names);
        block
    }

    /// 通过数据行创建，自动生成列元数据（TS build 的对象数组形式）。
    pub fn build_from_rows(block_id: &str, rows: Vec<Map<String, Value>>) -> EiBlock {
        let mut block = EiBlock::new(block_id);
        block.push_data(rows, true);
        block
    }

    /// 通过列中英文名数组创建（TS buildByNames）。
    pub fn build_by_names(block_id: &str, enames: &[&str], cnames: &[&str]) -> EiBlock {
        let mut block = EiBlock::new(block_id);
        for (i, ename) in enames.iter().enumerate() {
            let mut col = EiColumn::new(*ename, "string");
            if let Some(cname) = cnames.get(i) {
                col.desc_name = cname.to_string();
            }
            block.add_column(col, None);
        }
        block
    }
}

/// get_block 的寻址方式：按块名或按索引号（索引基于块名的字典序；前端按插入顺序）。
pub enum BlockRef<'a> {
    Name(&'a str),
    Index(usize),
}

impl<'a> From<&'a str> for BlockRef<'a> {
    fn from(name: &'a str) -> Self {
        BlockRef::Name(name)
    }
}

impl<'a> From<&'a String> for BlockRef<'a> {
    fn from(name: &'a String) -> Self {
        BlockRef::Name(name)
    }
}

impl From<usize> for BlockRef<'_> {
    fn from(index: usize) -> Self {
        BlockRef::Index(index)
    }
}

/// EiInfo 主结构（对应前端 EI.EIInfo）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EiInfo {
    #[serde(default)]
    pub context: Map<String, Value>,
    #[serde(default)]
    pub sys: SysInfo,
    #[serde(default)]
    pub blocks: BTreeMap<String, EiBlock>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub desc_name: String,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub msg_key: String,
    #[serde(default)]
    pub detail_msg: String,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub ext_attr: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ed_info: Option<EdInfo>,
}

impl EiInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_ed(&mut self, ed: EdInfo) {
        self.ed_info = Some(ed);
    }

    /// 添加 block 块；block_name 不传时以 block.name 作为键名。
    pub fn add_block(&mut self, mut block: EiBlock, block_name: Option<&str>) -> &mut EiBlock {
        let key = block_name
            .map(str::to_owned)
            .unwrap_or_else(|| block.name.clone());
        if block.name.is_empty() {
            block.name = key.clone();
        }
        self.blocks.insert(key.clone(), block);
        self.blocks.get_mut(&key).expect("刚插入的 block 必然存在")
    }

    pub fn contains(&self, block_name: &str) -> bool {
        self.blocks.contains_key(block_name)
    }

    pub fn remove(&mut self, block_name: &str) -> Option<EiBlock> {
        self.blocks.remove(block_name)
    }

    /// 重命名 block（键名与块自身的 name 一并更新）。
    pub fn rename_block(&mut self, old_block_name: &str, new_block_name: &str) -> Option<&EiBlock> {
        let mut block = self.blocks.remove(old_block_name)?;
        block.name = new_block_name.to_string();
        self.blocks.insert(new_block_name.to_string(), block);
        self.blocks.get(new_block_name)
    }

    /// 按块名（&str）或索引号（usize）获取 block。
    pub fn get_block<'a, R: Into<BlockRef<'a>>>(&self, block_info: R) -> Option<&EiBlock> {
        match block_info.into() {
            BlockRef::Name(name) => self.blocks.get(name),
            BlockRef::Index(index) => self.blocks.values().nth(index),
        }
    }

    pub fn get_block_mut<'a, R: Into<BlockRef<'a>>>(&mut self, block_info: R) -> Option<&mut EiBlock> {
        match block_info.into() {
            BlockRef::Name(name) => self.blocks.get_mut(name),
            BlockRef::Index(index) => self.blocks.values_mut().nth(index),
        }
    }

    /// 反序列化后调用：将 blocks 的键名回填到各 block.name（name 不随 JSON 传输）。
    pub fn sync_block_names(&mut self) {
        for (key, block) in self.blocks.iter_mut() {
            if block.name.is_empty() {
                block.name = key.clone();
            }
        }
    }

    /// 三段式取数：blockName-rowNum-colName，如 "result-0-siteArticleButton"；取不到返回 None。
    pub fn get(&self, key: &str) -> Option<&Value> {
        let (block_name, row, col) = parse_triple(key)?;
        self.blocks.get(block_name)?.get_cell(row, col)
    }

    /// 三段式设值：blockName-rowNum-colName；block 不存在时返回 false，行号超出时自动补空行。
    pub fn set(&mut self, key: &str, value: Value) -> bool {
        let Some((block_name, row, col)) = parse_triple(key) else {
            return false;
        };
        let Some(block) = self.blocks.get_mut(block_name) else {
            return false;
        };
        block.set_cell(row, col, value);
        true
    }

    /// 序列化为 JSON（等价前端的 toJSON）。
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// 解析三段式 key：blockName-rowNum-colName（列名允许含 '-'）。
fn parse_triple(key: &str) -> Option<(&str, usize, &str)> {
    let mut parts = key.splitn(3, '-');
    let block_name = parts.next()?;
    let row: usize = parts.next()?.parse().ok()?;
    let col = parts.next()?;
    if block_name.is_empty() || col.is_empty() {
        return None;
    }
    Some((block_name, row, col))
}
