use model::item::Item;
use std::collections::HashMap;

/// InMemoryStorage holds key-value pairs before they are persisted
/// to disk. All mutable operations occur here, and reads should
/// first check here before reading from disk.
pub struct InMemoryStorage {
    data: HashMap<String, Item>,
}

impl InMemoryStorage {
    pub fn new() -> InMemoryStorage {
        InMemoryStorage { data: HashMap::new() }
    }

    /// Get a value for a given key from the in memory storage map.
    pub fn get(&self, key: &String) -> Option<&Item> {
        debug!("Retrieving value for key: {}", key);
        self.data.get(key)
    }

    /// Insert a value for a given key into the map, returning the optional
    /// previously existing value.
    pub fn insert(&mut self, item: &Item) -> &Item {
        debug!("Inserting key {} and value {}", item.key(), item.val());
        self.data.insert(item.key().clone(), item.clone());
        self.get(item.key()).unwrap()
    }

    /// Remove a value for a given key from the map, returning the optional
    /// previously existing value.
    pub fn remove(&mut self, key: &String) -> Option<Item> {
        debug!("Removing key {}", key);
        self.data.remove(key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use model::item::Item;

    #[test]
    fn test_insert_get_remove_get() {
        let mut store = InMemoryStorage::new();

        let item1 = Item::new(String::from("a"), String::from("b"));
        let item2 = Item::new(String::from("a"), String::from("c"));
        assert_eq!(store.insert(&item1), &item1);
        assert_eq!(store.get(&String::from("a")), Option::Some(&item1));
        assert_eq!(store.insert(&item2), &item2);
        assert_eq!(store.remove(&String::from("a")), Option::Some(item2));
        assert_eq!(store.get(&String::from("a")), Option::None);
    }
}
