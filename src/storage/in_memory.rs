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

        // An item can be in the map with a status of deleted, which means
        // it will soon be blown away but still stays in the map for
        // a minute until we can safely clean it up. An item with deleted
        // status should be treated as not there for all consumers.
        match self.data.get(key) {
            Some(item) => {
                if item.deleted() {
                    return None;
                }
                Some(item)
            }
            None => None,
        }
    }

    /// Get an item out of the map as a mutable borrow.
    fn get_mut(&mut self, key: &String) -> Option<&mut Item> {
        self.data.get_mut(key)
    }

    /// Insert a value for a given key into the map, returning the optional
    /// previously existing value.
    pub fn insert(&mut self, item: &Item) -> &Item {
        debug!("Inserting key {} and value {}", item.key(), item.val());
        self.data.insert(item.key().clone(), item.clone());
        self.get(item.key()).unwrap()
    }

    /// Set the deletion status of an item to true, but not yet deleting it
    /// from the map.
    pub fn remove(&mut self, key: &String) -> Option<Item> {
        debug!("Removing key {}", key);

        let ref mut item = match self.get_mut(key) {
            Some(item) => item,
            None => return None, 
        };

        item.set_deleted(true);
        Some(item.clone())
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

        let deleted_item2 = store.remove(&String::from("a")).unwrap();
        assert_eq!(item2.deleted(), false);
        assert_eq!(deleted_item2.deleted(), true);
        assert_eq!(item2.key(), deleted_item2.key());
        assert_eq!(item2.val(), deleted_item2.val());

        assert_eq!(store.get(&String::from("a")), Option::None);
    }
}
