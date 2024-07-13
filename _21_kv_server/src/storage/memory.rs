use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use crate::{KvError, Kvpair, Storage, Value};

/// 使用 DashMap 构建的 MemTable，实现了 Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,

}

impl MemTable {
    /// 创建一个默认的 MemTable
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果名为 name 的 hash table 不存在，则创建，否则返回
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
            Some(table) => {
                table
            }
        }
    }
}

impl Storage for MemTable {
    /// 如果 Value 的路径不一样，编译无法通过
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);

        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError> {
        todo!()
    }
}