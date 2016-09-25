use std::collections::HashMap;

/// InMemoryStorage holds key-value pairs before they are persisted
/// to disk. All mutable operations occur here, and reads should
/// first check here before reading from disk.
pub struct InMemoryStorage {
    data: HashMap<String, String>,
}

impl InMemoryStorage {
    pub fn new() -> InMemoryStorage {
        InMemoryStorage { data: HashMap::new() }
    }

    /// Get a value for a given key from the in memory storage map.
    pub fn get(&self, key: &String) -> Option<&String> {
        debug!("Retrieving value for key: {}", key);
        self.data.get(key)
    }

    /// Insert a value for a given key into the map, returning the optional
    /// previously existing value.
    pub fn insert(&mut self, key: String, val: String) -> Option<String> {
        debug!("Inserting key {} and value {}", key, val);
        self.data.insert(key, val)
    }

    /// Remove a value for a given key from the map, returning the optional
    /// previously existing value.
    pub fn remove(&mut self, key: &String) -> Option<String> {
        debug!("Removing key {}", key);
        self.data.remove(key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert_get_remove_get() {
        let mut store = InMemoryStorage::new();
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
}
