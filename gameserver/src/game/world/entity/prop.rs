use proto::{MotionInfo, SceneEntityInfo, ScenePropInfo, Vector, VectorBin};

use super::{EntityType, SceneEntity};

pub struct SceneProp {
    position: VectorBin,
    rotation: VectorBin,
    entity_id: u32,
    group_id: u32,
    inst_id: u32,
    prop_id: u32,
    pub state: u32,
}

impl SceneProp {
    pub fn new(entity_id: u32, group_id: u32, inst_id: u32, prop_id: u32) -> Self {
        Self {
            entity_id,
            group_id,
            inst_id,
            prop_id,
            state: 1,
            position: VectorBin::default(),
            rotation: VectorBin::default(),
        }
    }
}

impl SceneEntity for SceneProp {
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
            group_id: self.group_id,
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
            inst_id: self.inst_id,
            prop: Some(ScenePropInfo {
                prop_id: self.prop_id,
                prop_state: self.state,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Prop
    }
}
