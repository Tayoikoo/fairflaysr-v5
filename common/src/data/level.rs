use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeLevelFloorInfo {
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    pub start_group_index: u32,
    #[serde(rename = "StartAnchorID")]
    pub start_anchor_id: u32,
    pub group_instance_list: Vec<RuntimeGroupInstanceInfo>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeGroupInstanceInfo {
    pub group_path: String,
    pub name: Option<String>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub is_delete: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct RuntimeGroupInfo {
    pub load_on_initial: bool,
    pub load_side: GroupLoadSide,        
    pub group_name: Option<String>,
    pub anchor_list: Option<Vec<LevelAnchorInfo>>,
    pub prop_list: Option<Vec<LevelPropInfo>>,
}

#[derive(Default, Deserialize, PartialEq, Eq)]
pub enum GroupLoadSide {
    #[default]
    Client,
    Server,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelAnchorInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
    pub is_delete: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct LevelPropInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
    #[serde(rename = "PropID")]
    pub prop_id: u32,
    pub load_on_initial: bool,
    #[serde(rename = "CocoonID")]
    pub cocoon_id: u32,
}
