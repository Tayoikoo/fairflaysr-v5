use crate::{
    game::world::{entity::SceneEntity, GameWorld},
    util,
};

use super::*;
use common::data::EXCEL_COLLECTION;
use proto::*;

pub struct SceneManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
    world: AtomicRefCell<GameWorld>,
}

impl SceneManager {
    const MAX_SECTION_ID: u32 = 100;

    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self {
            player_info,
            world: AtomicRefCell::new(GameWorld::default()),
        }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();

        player_info.data.scene_bin = Some(PlayerSceneCompBin {
            cur_entry_id: 2033201,//2010101,
            cur_position: Some(MotionBin::default()),
            scene_list: Vec::new(),
        });
    }

    pub fn enter_scene(&mut self, entry_id: u32) -> Result<SceneInfo, Retcode> {
        self.load_scene(entry_id)?;
        Ok(self.cur_scene_info_proto())
    }

    pub fn entity_move(&mut self, entity_motion: &EntityMotion) {
        let world = self.world.borrow();

        if world.is_actor(entity_motion.entity_id) {
            let mut player_info = self.player_info.borrow_mut();
            let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

            let motion = entity_motion.motion.as_ref().unwrap();
            let pos = motion.pos.as_ref().unwrap();
            let rot = motion.rot.as_ref().unwrap();

            println!(
                "Entity Position - X: {}, Y: {}, Z: {}, Rotation - X: {}, Y: {}, Z: {}",
                pos.x, pos.y, pos.z, rot.x, rot.y, rot.z
            );

            scene_comp.cur_position = Some(MotionBin {
                pos: Some(VectorBin {
                    x: pos.x,
                    y: pos.y,
                    z: pos.z,
                }),
                rot: Some(VectorBin {
                    x: rot.x,
                    y: rot.y,
                    z: rot.z,
                }),
            });
        }
    }

    pub fn refresh_actor_group(&mut self) -> Option<SceneGroupRefreshInfo> {
        let player_uid = self.player_info.borrow().uid;

        self.world
            .borrow_mut()
            .scene_group_list
            .retain(|g| g.id != 0);

        self.cur_lineup_avatars().iter().for_each(|id| {
            self.world
                .borrow_mut()
                .add_player_actor(player_uid, *id, None, None)
        });

        let world = self.world.borrow();
        world
            .scene_group_list
            .iter()
            .find(|g| g.id == 0)
            .map(|g| SceneGroupRefreshInfo {
                group_id: g.id,
                state: 1,
                refresh_type: SceneGroupRefreshType::None.into(),
                refresh_entity_list: g
                    .actor_list
                    .iter()
                    .map(|a| SceneGroupEntityRefreshInfo {
                        refresh_entity_info: Some(a.scene_entity_info_proto()),
                        ..Default::default()
                    })
                    .collect(),
            })
    }

    fn load_scene(&mut self, entry_id: u32) -> Result<(), Retcode> {
        let cur_avatar_id_list = self.cur_lineup_avatars();

        let mut player_info = self.player_info.borrow_mut();
        let player_uid = player_info.uid;
        let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

        let Some(entrance_config) = EXCEL_COLLECTION
            .map_entrance_configs
            .iter()
            .find(|c| c.id == entry_id)
        else {
            return Err(Retcode::RetAdventureMapNotExist);
        };

        let mut world =
            GameWorld::new(entry_id, entrance_config.plane_id, entrance_config.floor_id);

        let (pos, rot) = if scene_comp.cur_entry_id == entry_id {
            let motion = scene_comp.cur_position.as_ref().unwrap();
            (motion.pos.clone(), motion.rot.clone())
        } else {
            (None, None)
        };

        cur_avatar_id_list
            .iter()
            .for_each(|id| world.add_player_actor(player_uid, *id, pos.as_ref(), rot.as_ref()));

        world.init_groups();

        self.world = AtomicRefCell::new(world);
        if let Some(scene_bin) = scene_comp.scene_list.iter_mut().find(|s| {
            s.plane_id == entrance_config.plane_id && s.floor_id == entrance_config.floor_id
        }) {
            scene_bin.last_enter_time = util::cur_timestamp_seconds() as i64;
        } else {
            scene_comp.scene_list.push(SceneBin {
                plane_id: entrance_config.plane_id,
                floor_id: entrance_config.floor_id,
                lighten_section_list: Vec::new(),
                unlocked_teleport_list: Vec::new(),
                last_enter_time: util::cur_timestamp_seconds() as i64,
            });
        }

        scene_comp.cur_entry_id = entry_id;
        Ok(())
    }

    pub fn enter_section(&self, section_id: u32) {
        if section_id < Self::MAX_SECTION_ID {
            let world = self.world.borrow();

            let mut player_info = self.player_info.borrow_mut();
            let scene_comp = player_info.data.scene_bin.as_mut().unwrap();

            if let Some(scene_bin) = scene_comp
                .scene_list
                .iter_mut()
                .find(|s| s.plane_id == world.plane_id && s.floor_id == world.floor_id)
            {
                if !scene_bin.lighten_section_list.contains(&section_id) {
                    scene_bin.lighten_section_list.push(section_id);
                }
            }
        }
    }

    pub fn cur_scene_info_proto(&self) -> SceneInfo {
        let world = self.world.borrow();

        let player_info = self.player_info.borrow();
        let scene_comp = player_info.data.scene_bin.as_ref().unwrap();
        let scene_bin = scene_comp
            .scene_list
            .iter()
            .find(|s| s.plane_id == world.plane_id && s.floor_id == world.floor_id)
            .unwrap();

        SceneInfo {
            default_pos: 1,
            plane_id: world.plane_id,
            floor_id: world.floor_id,
            entry_id: world.entry_id,
            game_mode_type: 1,
            leader_entity_id: world.leader_entity_id,
            scene_group_list: world.scene_group_list.iter().map(|g| g.info()).collect(),
            lighten_section_list: scene_bin.lighten_section_list.clone(),
            ..Default::default()
        }
    }

    pub fn cur_entry_id(&self) -> u32 {
        let player_info = self.player_info.borrow();
        let scene_comp = player_info.data.scene_bin.as_ref().unwrap();

        scene_comp.cur_entry_id
    }

    fn cur_lineup_avatars(&self) -> Vec<u32> {
        let player_info = self.player_info.borrow();
        let lineup_comp = player_info.data.lineup_bin.as_ref().unwrap();

        let cur_lineup = lineup_comp
            .lineup_list
            .iter()
            .find(|l| l.index == lineup_comp.cur_lineup_index)
            .unwrap();
        cur_lineup.avatar_list.iter().map(|a| a.avatar_id).collect()
    }
}
