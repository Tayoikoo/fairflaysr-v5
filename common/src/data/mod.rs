mod excels;
pub mod level;

use std::collections::HashMap;

pub use excels::*;
use lazy_static::lazy_static;
use serde_json::from_str;

use self::level::{RuntimeGroupInfo, RuntimeLevelFloorInfo};

pub fn init_assets() {
    tracing::info!("Preparing assets...");
    tracing::info!("Loaded {} excel tables", EXCEL_COLLECTION.table_count());
    tracing::info!(
        "Loaded {} floors, {} groups",
        LEVEL_TABLE.level_floors.len(),
        LEVEL_TABLE.level_groups.len()
    );
}

lazy_static! {
    pub static ref EXCEL_COLLECTION: ExcelCollection = ExcelCollection::new();
    pub static ref LEVEL_TABLE: LevelTable = LevelTable::new();
}

pub struct ExcelCollection {
    pub avatar_configs: Vec<AvatarConfig>,
    pub avatar_skill_tree_configs: Vec<AvatarSkillTreeConfig>,
    pub item_configs: Vec<ItemConfig>,
    pub equipment_configs: Vec<EquipmentConfig>,
    pub relic_configs: Vec<RelicConfig>,
    pub map_entrance_configs: Vec<MapEntranceConfig>,
    pub tutorial_configs: Vec<TutorialConfig>,
    pub tutorial_guide_group_configs: Vec<TutorialGuideGroupConfig>,
}

impl ExcelCollection {
    fn new() -> Self {
        Self {
            avatar_configs: from_str(&load_asset("ExcelOutput/AvatarConfig.json")).unwrap(),
            avatar_skill_tree_configs: from_str(&load_asset(
                "ExcelOutput/AvatarSkillTreeConfig.json",
            ))
            .unwrap(),
            item_configs: from_str(&load_asset("ExcelOutput/ItemConfig.json")).unwrap(),
            equipment_configs: from_str(&load_asset("ExcelOutput/EquipmentConfig.json")).unwrap(),
            relic_configs: from_str(&load_asset("ExcelOutput/RelicConfig.json")).unwrap(),
            map_entrance_configs: from_str(&load_asset("ExcelOutput/MapEntranceConfig.json"))
                .unwrap(),
            tutorial_configs: from_str(&load_asset("ExcelOutput/TutorialConfig.json")).unwrap(),
            tutorial_guide_group_configs: from_str(&load_asset(
                "ExcelOutput/TutorialGuideGroupConfig.json",
            ))
            .unwrap(),
        }
    }

    pub fn table_count(&self) -> usize {
        8
    }
}

pub struct LevelTable {
    pub level_floors: HashMap<u32, RuntimeLevelFloorInfo>,
    pub level_groups: HashMap<u64, RuntimeGroupInfo>,
}

impl LevelTable {
    fn new() -> Self {
        let id_list = EXCEL_COLLECTION
            .map_entrance_configs
            .iter()
            .map(|c| (c.plane_id, c.floor_id));

        let mut level_floors = HashMap::new();
        for (plane_id, floor_id) in id_list {
            if !level_floors.contains_key(&floor_id) {
                level_floors.insert(
                    floor_id,
                    from_str::<RuntimeLevelFloorInfo>(&load_asset(
                        format!("LevelOutput/RuntimeFloor/P{plane_id}_F{floor_id}.json").as_str(),
                    ))
                    .unwrap(),
                );
            }
        }

        let mut level_groups = HashMap::new();
        for floor_info in level_floors.values() {
            for group_instance in &floor_info.group_instance_list {
                level_groups.insert(
                    ((floor_info.floor_id as u64) << 32) | group_instance.id as u64,
                    from_str::<RuntimeGroupInfo>(&load_asset(&group_instance.group_path)).unwrap(),
                );
            }
        }

        Self {
            level_floors,
            level_groups,
        }
    }

    pub fn get_level_group(&self, floor_id: u32, group_id: u32) -> Option<&RuntimeGroupInfo> {
        self.level_groups
            .get(&((floor_id as u64) << 32 | group_id as u64))
    }
}

fn load_asset(path: &str) -> String {
    let path = if path.starts_with("Config/") {
        path.strip_prefix("Config/").unwrap()
    } else {
        path
    };

    std::fs::read_to_string(format!("assets/{path}")).unwrap()
}
