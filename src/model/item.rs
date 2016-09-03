use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
/// A representation of an item stored in memory or on disk.
pub struct Item {
    key: String,
    val: String,
}

impl Item {
    /// Constructs a new item for a given key and value
    pub fn new(key: String, val: String) -> Item {
        Item {
            key: key,
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
    pub fn to_json(&self) -> Result<String, json::EncoderError> {
        json::encode(&self)
    }

    /// Takes a json string and creates an item
    pub fn from_json(json: &str) -> Result<Item, json::DecoderError> {
        json::decode(&json)
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

        let encoded = item.to_json().unwrap();
        assert_eq!("{\"key\":\"k\",\"val\":\"v\"}", encoded);

        let decoded: Item = Item::from_json(&encoded).unwrap();
        assert_eq!(item.key(), decoded.key());
        assert_eq!(item.val(), decoded.val());
    }
}
