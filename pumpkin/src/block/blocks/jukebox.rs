use crate::block::block_manager::BlockActionResult;
use crate::block::pumpkin_block::PumpkinBlock;
use crate::entity::player::Player;
use crate::server::Server;
use crate::world::World;
use async_trait::async_trait;
use pumpkin_core::math::position::WorldPosition;
use pumpkin_macros::pumpkin_block;
use pumpkin_registry::SYNCED_REGISTRIES;
use pumpkin_world::block::block_entity::{BlockEntity, BlockEntityType, RecordItem};
use pumpkin_world::item::item_registry::{get_item, get_record_item, Item};
use std::sync::Arc;

#[pumpkin_block("minecraft:jukebox")]
pub struct JukeboxBlock;

#[async_trait]
impl PumpkinBlock for JukeboxBlock {
    async fn on_use<'a>(&self, player: &Player, location: WorldPosition, _server: &Server) {
        let world = &player.living_entity.entity.world;

        let block_entity = world.get_block_entity(&location).await;
        let Some(block_entity) = block_entity else {
            return;
        };

        let record_item = match block_entity.data {
            BlockEntityType::Jukebox { record_item, .. } => record_item,
            _ => return,
        };

        dbg!(&record_item);

        if let Some(record_item) = record_item {
            //TODO: The record would drop here for now just give the disk to the player
            let item = get_record_item(record_item.id.as_str());

            let Some(item) = item else {
                return;
            };

            player.give_items(item, 1).await;

            world.stop_record(location).await;

            Self::write_block_entity(world, location, None, None).await;
        }
    }

    async fn on_use_with_item<'a>(
        &self,
        player: &Player,
        location: WorldPosition,
        item: &Item,
        _server: &Server,
    ) -> BlockActionResult {
        let world = &player.living_entity.entity.world;

        let Some(jukebox_playable) = &item.components.jukebox_playable else {
            return BlockActionResult::Continue;
        };

        let Some(song) = jukebox_playable.song.split(':').nth(1) else {
            return BlockActionResult::Continue;
        };

        let Some(jukebox_song) = SYNCED_REGISTRIES.jukebox_song.get_index_of(song) else {
            log::error!("Jukebox playable song not registered!");
            return BlockActionResult::Continue;
        };

        //TODO: Update block state and block nbt

        world.play_record(jukebox_song as i32, location).await;

        Self::write_block_entity(
            world,
            location,
            Some(RecordItem {
                count: 1,
                id: jukebox_playable.song.clone(),
            }),
            Some(0),
        )
        .await;

        BlockActionResult::Consume
    }

    async fn on_placed<'a>(&self, player: &Player, location: WorldPosition, _server: &Server) {
        let world = &player.living_entity.entity.world;
        Self::write_block_entity(world, location, None, None).await;
    }

    async fn on_broken<'a>(&self, player: &Player, location: WorldPosition, _server: &Server) {
        // For now just stop the music at this position
        let world = &player.living_entity.entity.world;

        world.stop_record(location).await;

        world.remove_block_entity(location).await;
    }
}

impl JukeboxBlock {
    pub async fn write_block_entity(
        world: &Arc<World>,
        location: WorldPosition,
        record_item: Option<RecordItem>,
        ticks_since_song_started: Option<i64>,
    ) {
        let block_entity = BlockEntity {
            x: location.0.x,
            y: location.0.y,
            z: location.0.z,
            data: BlockEntityType::Jukebox {
                record_item,
                ticks_since_song_started,
            },
        };

        world
            .create_or_update_block_entity(location, block_entity)
            .await;
    }
}
