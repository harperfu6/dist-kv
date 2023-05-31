use std::collections::HashMap;

use chashmap::CHashMap;

#[derive(Clone)]
pub struct KvElement {
    data: String,
}

#[derive(Clone)]
pub struct KvStore {
    container: CHashMap<String, KvElement>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            container: CHashMap::new(),
        }
    }

    pub fn set(&self, data: HashMap<String, String>) {
        for (k, v) in data {
            self.container.insert(k, KvElement { data: v });
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.container.get(&key) {
            Some(v) => Some(v.data.clone()),
            None => None,
        }
    }

    pub fn drop(&self, key: String) {
        self.container.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let kv = KvStore::new();
        kv.set(HashMap::from([("key".to_string(), "value".to_string())]));
        assert_eq!(kv.get("key".to_string()), Some("value".to_string()));
    }

    #[test]
    fn test_drop() {
        let kv = KvStore::new();
        kv.set(HashMap::from([("key".to_string(), "value".to_string())]));
        kv.drop("key".to_string());
        assert_eq!(kv.get("key".to_string()), None);
    }
}
