use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntityItem {
    count: Option<u8>,
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
    // #[serde(rename = "id")]
    // pub id: String,
    #[serde(flatten)]
    pub data: BlockEntityType
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "id")]
pub enum BlockEntityType {
    
    #[serde(rename = "minecraft:jukebox")]
    Jukebox {
        #[serde(rename = "RecordItem")]
        record_item: RecordItem,
        ticks_since_song_started: i64,
    },
    
    #[serde(other)]
    Unknown
}


/// --- Entity specific structs ---

/// Jukebox record item
#[derive(Debug, Clone, Deserialize)]
pub struct RecordItem {
    count: u8,
    id: String,
}