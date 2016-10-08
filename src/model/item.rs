include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use serde_json;
use serde_json::de;
use serde_json::ser;

/// Implements the Item struct defined in src/serde_types.in.rs 
/// (thanks serde youre the best) 
impl Item {
    /// Constructs a new item for a given key and value
    pub fn new(key: String, val: String) -> Item {
        Item {
            key: key.to_lowercase(),
            val: val,
        }
    }

    /// Accesses the key of an item
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Accesses the value of an item
    pub fn val(&self) -> &String {
        &self.val
    }

    /// Takes an item and coverts it to json
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        ser::to_string(&self)
    }

    /// Takes a json string and creates an item
    pub fn from_json(json: &str) -> Result<Item, serde_json::Error> {
        let res: Result<Item, serde_json::Error> = de::from_str(json);
        match res {
            Ok(mut item) => {
                // maybe worth checking if the key is already lowercased
                item.key = item.key.to_lowercase();
                Ok(item)
            },
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize_and_deserialize_item() {
        let item = Item::new(String::from("k"), String::from("v"));
        assert_eq!(item.key(), &String::from("k"));
        assert_eq!(item.val(), &String::from("v"));

        let decoded: Item = Item::from_json(&item.to_json().unwrap()).unwrap();
        assert_eq!(item.key(), decoded.key());
        assert_eq!(item.val(), decoded.val());
    }
}
