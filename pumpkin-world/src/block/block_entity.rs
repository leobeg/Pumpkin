use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntityItem {
    count: u8,
    slot: Option<u8>,
    #[serde(rename = "id")]
    id: String,
    //tag  - Currently not needed
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockEntity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[serde(flatten)]
    pub data: BlockEntityType
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "id")]
pub enum BlockEntityType {
    #[serde(rename = "minecraft:jukebox")]
    #[serde(rename_all = "PascalCase")]
    Jukebox {
        is_playing: bool,
        record_item: BlockEntityItem,

    },
    #[serde(other)]
    Unknown
}