use proto::{SceneEntityInfo, VectorBin};

mod actor;
mod prop;

pub use actor::SceneActor;
pub use prop::SceneProp;

#[allow(dead_code)]
pub trait SceneEntity {
    fn position(&self) -> VectorBin;
    fn rotation(&self) -> VectorBin;
    fn set_position(&mut self, x: i32, y: i32, z: i32);
    fn set_rotation(&mut self, x: i32, y: i32, z: i32);
    fn entity_id(&self) -> u32;
    fn scene_entity_info_proto(&self) -> SceneEntityInfo;
    fn entity_type(&self) -> EntityType;
}

#[repr(u32)]
pub enum EntityType {
    Actor = 1,
    Prop = 2,
}
