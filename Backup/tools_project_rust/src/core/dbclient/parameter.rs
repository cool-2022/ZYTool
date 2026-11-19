// 命名参数集合（对应 C++ CDataParameter / CDataParameterCollection）。
// 参数名大小写不敏感，保持插入顺序（存储过程按顺序传参）。
use super::value::{DbValue, FromDbValue};

/// 对应 C++ DATAPARAMETERIOENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamDirection {
    Input,
    Output,
    InputOutput,
}

#[derive(Debug, Clone)]
pub struct DataParameter {
    pub name: String,
    pub value: DbValue,
    pub direction: ParamDirection,
    pub size: i32,
}

impl DataParameter {
    pub fn get<T: FromDbValue>(&self) -> Option<T> {
        T::from_db_value(&self.value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParameterCollection {
    items: Vec<DataParameter>,
}

fn key(name: &str) -> String {
    name.trim().trim_start_matches('@').to_ascii_uppercase()
}

impl ParameterCollection {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置输入参数；同名参数覆盖值但保留原位置
    pub fn set(&mut self, name: &str, value: impl Into<DbValue>) -> &mut Self {
        self.set_with(name, value, ParamDirection::Input, 0)
    }

    pub fn set_with(
        &mut self,
        name: &str,
        value: impl Into<DbValue>,
        direction: ParamDirection,
        size: i32,
    ) -> &mut Self {
        let k = key(name);
        let value = value.into();
        match self.items.iter_mut().find(|p| key(&p.name) == k) {
            Some(p) => {
                p.value = value;
                p.direction = direction;
                p.size = size;
            }
            None => self.items.push(DataParameter {
                name: name.trim().trim_start_matches('@').to_string(),
                value,
                direction,
                size,
            }),
        }
        self
    }

    pub fn set_null(&mut self, name: &str) -> &mut Self {
        self.set(name, DbValue::Null)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&DataParameter> {
        let k = key(name);
        self.items.iter().find(|p| key(&p.name) == k)
    }

    pub fn get_value(&self, name: &str) -> Option<&DbValue> {
        self.get(name).map(|p| &p.value)
    }

    /// 对应 C++ GetInt32(name)/GetString(name)/...：NULL 或不存在返回 None
    pub fn get_as<T: FromDbValue>(&self, name: &str) -> Option<T> {
        self.get(name).and_then(DataParameter::get)
    }

    pub fn remove(&mut self, name: &str) -> Option<DataParameter> {
        let k = key(name);
        let idx = self.items.iter().position(|p| key(&p.name) == k)?;
        Some(self.items.remove(idx))
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DataParameter> {
        self.items.iter()
    }
}
