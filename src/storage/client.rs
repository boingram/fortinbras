use model::item::Item;
use storage::commit_log::CommitLog;
use storage::in_memory::InMemoryStorage;

/// StorageClient exposes a simple interface to insert, read, and remove
/// keys from Fortinbras.
pub struct StorageClient {
    commit_log: CommitLog,
    in_memory: InMemoryStorage,
}

impl StorageClient {
    /// Creates a new instance of the StorageClient with newly initialized
    /// in-memory storage.
    pub fn new() -> StorageClient {
        StorageClient {
            commit_log: CommitLog::init(".data"),
            in_memory: InMemoryStorage::new(),
        }
    }

    /// Given a &String key, retrieve an optional result.
    pub fn get(&self, key: &String) -> Option<&Item> {
        self.in_memory.get(&key.to_lowercase())
    }

    /// Insert a value for a given key, returning the written item.
    pub fn insert(&mut self, item: &Item) -> Option<&Item> {
        // option isn't necessarily the best thing at the moment, but
        // it will be so I'm going to leave the return type an option
        // for now

        match self.commit_log.write(item) {
            Ok(_) => Some(self.in_memory.insert(item)),
            Err(e) => {
                error!("Error writing item with key {} to commit log: {}",
                       item.key(),
                       e);
                None
            }
        }
    }

    /// Remove a key, returning the optional previously existing value.
    pub fn remove(&mut self, key: &String) -> Option<Item> {
        self.in_memory.remove(&key.to_lowercase())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use model::item::Item;

    #[test]
    fn test_insert_get_remove_get() {
        let mut store = StorageClient::new();

        let item1 = Item::new(String::from("a"), String::from("b"));
        let item2 = Item::new(String::from("a"), String::from("c"));
        assert_eq!(store.insert(&item1), Option::Some(&item1));
        assert_eq!(store.get(&String::from("a")), Option::Some(&item1));
        assert_eq!(store.insert(&item2), Option::Some(&item2));

        let deleted_item2 = store.remove(&String::from("a")).unwrap();
        assert_eq!(item2.deleted(), false);
        assert_eq!(deleted_item2.deleted(), true);
        assert_eq!(item2.key(), deleted_item2.key());
        assert_eq!(item2.val(), deleted_item2.val());

        assert_eq!(store.get(&String::from("a")), Option::None);
    }

    #[test]
    fn test_lowercase_key_ops() {
        let mut store = StorageClient::new();
        store.insert(&Item::new(String::from("A"), String::from("b")));
        assert_eq!(store.get(&String::from("A")), store.get(&String::from("a")));
    }
}
