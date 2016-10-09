/// A representation of an item stored in memory or on disk.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    key: String,
    val: String,

    #[serde(default)]
    deleted: bool,
}
