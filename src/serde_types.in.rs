/// A representation of an item stored in memory or on disk.
#[derive(Serialize, Deserialize)]
pub struct Item {
    key: String,
    val: String,
}
