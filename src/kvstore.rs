use std::collections::HashMap;

pub struct KvElement {
    data: String,
}

pub struct KvStore {
    container: HashMap<String, KvElement>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            container: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.container.insert(key, KvElement { data: value });
    }

    pub fn get(self, key: String) -> Option<String> {
        match self.container.get(&key) {
            Some(v) => Some(v.data.clone()),
            None => None,
        }
    }

    pub fn drop(&mut self, key: String) {
        self.container.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut kv = KvStore::new();
        kv.set("key".to_string(), "value".to_string());
        assert_eq!(kv.get("key".to_string()), Some("value".to_string()));
    }

    #[test]
    fn test_drop() {
        let mut kv = KvStore::new();
        kv.set("key".to_string(), "value".to_string());
        kv.drop("key".to_string());
        assert_eq!(kv.get("key".to_string()), None);
    }
}
