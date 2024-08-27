use proto::{MotionInfo, SceneActorInfo, SceneEntityInfo, Vector, VectorBin};

use super::{EntityType, SceneEntity};

pub struct SceneActor {
    position: VectorBin,
    rotation: VectorBin,
    uid: u32,
    avatar_id: u32,
    avatar_type: i32,
    map_layer: u32,
    entity_id: u32,
}

impl SceneActor {
    pub fn new(entity_id: u32, uid: u32, avatar_id: u32, avatar_type: i32) -> Self {
        Self {
            position: VectorBin::default(),
            rotation: VectorBin::default(),
            uid,
            avatar_id,
            avatar_type,
            map_layer: 1,
            entity_id,
        }
    }
}

impl SceneEntity for SceneActor {
    fn position(&self) -> VectorBin {
        self.position.clone()
    }

    fn rotation(&self) -> VectorBin {
        self.rotation.clone()
    }

    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        self.position = VectorBin { x, y, z }
    }

    fn set_rotation(&mut self, x: i32, y: i32, z: i32) {
        self.rotation = VectorBin { x, y, z }
    }

    fn entity_id(&self) -> u32 {
        self.entity_id
    }

    fn scene_entity_info_proto(&self) -> SceneEntityInfo {
        SceneEntityInfo {
            motion: Some(MotionInfo {
                pos: Some(Vector {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z,
                }),
                rot: Some(Vector {
                    x: self.rotation.x,
                    y: self.rotation.y,
                    z: self.rotation.z,
                }),
            }),
            entity_id: self.entity_id,
            actor: Some(SceneActorInfo {
                uid: self.uid,
                map_layer: self.map_layer,
                base_avatar_id: self.avatar_id,
                avatar_type: self.avatar_type,
            }),
            ..Default::default()
        }
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Actor
    }
}
