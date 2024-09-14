pub mod entity;
mod group;

pub use group::SceneGroup;

use std::sync::atomic::{AtomicU32, Ordering};

use common::data::{
    level::{GroupLoadSide, LevelAnchorInfo},
    LEVEL_TABLE,
};
use proto::{AvatarType, VectorBin};

use self::entity::{EntityType, SceneActor, SceneEntity, SceneProp};

#[derive(Default)]
pub struct GameWorld {
    pub entry_id: u32,
    pub plane_id: u32,
    pub floor_id: u32,
    pub leader_entity_id: u32,
    pub scene_group_list: Vec<SceneGroup>,
    entity_counter: AtomicU32,
}

impl GameWorld {
    pub fn new(entry_id: u32, plane_id: u32, floor_id: u32) -> Self {
        Self {
            entry_id,
            plane_id,
            floor_id,
            leader_entity_id: 0,
            scene_group_list: Vec::new(),
            entity_counter: AtomicU32::new(0),
        }
    }

    pub fn is_actor(&self, entity_id: u32) -> bool {
        if let Some(group) = self.scene_group_list.iter().find(|g| g.id == 0) {
            group.actor_list.iter().any(|a| a.entity_id() == entity_id)
        } else {
            false
        }
    }

    pub fn add_player_actor(
        &mut self,
        uid: u32,
        avatar_id: u32,
        pos: Option<&VectorBin>,
        rot: Option<&VectorBin>,
    ) {
        let mut actor = SceneActor::new(
            EntityType::Actor as u32 * 1000000 + self.next_entity_id(),
            uid,
            avatar_id,
            AvatarType::AvatarFormalType.into(),
        );

        if let (Some(pos), Some(rot)) = (pos, rot) {
            actor.set_position(pos.x, pos.y, pos.z);
            actor.set_rotation(rot.x, rot.y, rot.z);
        } else {
            let anchor = self.get_player_start_position();
            actor.set_position(
                (anchor.pos_x * 1000.0) as i32,
                (anchor.pos_y * 1000.0) as i32,
                (anchor.pos_z * 1000.0) as i32,
            );
            actor.set_rotation(
                (anchor.rot_x * 1000.0) as i32,
                (anchor.rot_y * 1000.0) as i32,
                (anchor.rot_z * 1000.0) as i32,
            );
        }

        self.leader_entity_id = actor.entity_id();

        if let Some(group) = self.scene_group_list.iter_mut().find(|g| g.id == 0) {
            group.actor_list.push(actor);
        } else {
            self.scene_group_list.push(SceneGroup {
                id: 0,
                actor_list: vec![actor],
                prop_list: Vec::new(),
            });
        }
    }

    pub fn init_groups(&mut self) {
        let floor = LEVEL_TABLE.level_floors.get(&self.floor_id).unwrap();

        for group_instance in floor
            .group_instance_list
            .iter()
            .filter(|inst| !inst.is_delete)
        {
            let level_group = LEVEL_TABLE
                .get_level_group(self.floor_id, group_instance.id)
                .unwrap();

            if level_group.load_side == GroupLoadSide::Client {
                continue;
            }

            let mut prop_entity_list = Vec::new();
            if let Some(prop_list) = level_group.prop_list.as_ref() {
                for level_prop in prop_list {
                    if !level_prop.load_on_initial {
                        continue;
                    }

                    // For now, spawn only calyxes
                    if level_prop.cocoon_id == 0 {
                        continue;
                    }

                    let mut prop = SceneProp::new(
                        EntityType::Prop as u32 * 1000000 + self.next_entity_id(),
                        group_instance.id,
                        level_prop.id,
                        level_prop.prop_id,
                    );

                    prop.set_position(
                        (level_prop.pos_x * 1000.0 ) as i32,
                        (level_prop.pos_y * 1000.0 ) as i32,
                        (level_prop.pos_z * 1000.0 ) as i32,
                    );
                    prop.set_rotation(
                        (level_prop.rot_x * 1000.0 ) as i32,
                        (level_prop.rot_y * 1000.0 ) as i32,
                        (level_prop.rot_z * 1000.0 ) as i32,
                    );
                    // Debug prints
                    println!(
                        "Spawning prop: id={}, pos=({},{},{}), rot=({},{},{})",
                        level_prop.prop_id,
                        prop.position().x,
                        prop.position().y,
                        prop.position().z,
                        prop.rotation().x,
                        prop.rotation().y,
                        prop.rotation().z
                    );
                    prop_entity_list.push(prop);
                }
            }

            self.scene_group_list.push(SceneGroup {
                id: group_instance.id,
                prop_list: prop_entity_list,
                actor_list: Vec::new(),
            });
        }
    }

    fn get_player_start_position(&self) -> &LevelAnchorInfo {
        let floor = LEVEL_TABLE.level_floors.get(&self.floor_id).unwrap();
        let group_instance = &floor.group_instance_list[floor.start_group_index as usize];

        let group = LEVEL_TABLE
            .get_level_group(self.floor_id, group_instance.id)
            .unwrap();
        &group
            .anchor_list
            .as_ref()
            .unwrap()
            .iter()
            .find(|a| a.id == floor.start_anchor_id)
            .unwrap()
    }

    fn next_entity_id(&self) -> u32 {
        self.entity_counter.fetch_add(1, Ordering::SeqCst) + 1
    }
}
