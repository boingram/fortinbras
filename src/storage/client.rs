use storage::in_memory::InMemoryStorage;

/// StorageClient exposes a simple interface to insert, read, and remove
/// keys from Fortinbras.
pub struct StorageClient {
    in_memory: InMemoryStorage,
}

impl StorageClient {
    /// Creates a new instance of the StorageClient with newly initialized
    /// in-memory storage.
    pub fn new() -> StorageClient {
        StorageClient { in_memory: InMemoryStorage::new() }
    }

    /// Given a &String key, retrieve an optional result.
    pub fn get(&self, key: &String) -> Option<&String> {
        self.in_memory.get(&key.to_lowercase())
    }

    /// Insert a value for a given key, returning the optional previously
    /// existing value for the key.
    pub fn insert(&mut self, key: String, val: String) -> Option<String> {
        self.in_memory.insert(key.to_lowercase(), val)
    }

    /// Remove a key, returning the optional previously existing value.
    pub fn remove(&mut self, key: &String) -> Option<String> {
        self.in_memory.remove(&key.to_lowercase())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert_get_remove_get() {
        let mut store = StorageClient::new();
        assert_eq!(store.insert(String::from("a"), String::from("b")),
                   Option::None);
        assert_eq!(store.get(&String::from("a")),
                   Option::Some(&String::from("b")));
        assert_eq!(store.insert(String::from("a"), String::from("c")),
                   Option::Some(String::from("b")));
        assert_eq!(store.remove(&String::from("a")),
                   Option::Some(String::from("c")));
        assert_eq!(store.get(&String::from("a")), Option::None);
    }

    #[test]
    fn test_lowercase_key_ops() {
        let mut store = StorageClient::new();
        store.insert(String::from("A"), String::from("b"));
        assert_eq!(store.get(&String::from("A")), store.get(&String::from("a")));
    }
}
