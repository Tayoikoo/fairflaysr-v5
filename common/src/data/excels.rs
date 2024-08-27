use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarConfig {
    #[serde(rename = "AIPath")]
    pub aipath: String,
    pub action_avatar_head_icon_path: String,
    #[serde(rename = "AdventurePlayerID")]
    pub adventure_player_id: u32,
    pub assist_bg_offset: Vec<f64>,
    pub assist_offset: Vec<f64>,
    pub avatar_base_type: String,
    pub avatar_cutin_bg_img_path: String,
    pub avatar_cutin_front_img_path: String,
    pub avatar_cutin_img_path: String,
    pub avatar_cutin_intro_text: TextMapHashEntry,
    pub avatar_desc: TextMapHashEntry,
    pub avatar_drop_offset: Vec<f64>,
    pub avatar_full_name: TextMapHashEntry,
    pub avatar_gacha_result_img_path: String,
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
    pub avatar_initial_skin_desc: TextMapHashEntry,
    pub avatar_initial_skin_name: TextMapHashEntry,
    pub avatar_mini_icon_path: String,
    pub avatar_name: TextMapHashEntry,
    pub avatar_self_show_offset: Vec<f64>,
    pub avatar_side_icon_path: String,
    pub avatar_trial_offset: Vec<f64>,
    #[serde(rename = "AvatarVOTag")]
    pub avatar_votag: String,
    pub damage_type: String,
    pub damage_type_resistance: Vec<f64>,
    pub default_avatar_head_icon_path: String,
    pub default_avatar_model_path: String,
    pub exp_group: u32,
    pub json_path: String,
    pub manikin_json_path: String,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub player_card_offset: Vec<f64>,
    #[serde(rename = "RankIDList")]
    pub rank_idlist: Vec<u32>,
    pub rarity: String,
    pub release: bool,
    pub reward_list: Vec<ItemEntry>,
    pub reward_list_max: Vec<ItemEntry>,
    #[serde(rename = "SPNeed")]
    pub sp_need: FixPoint,
    pub side_avatar_head_icon_path: String,
    pub skill_list: Vec<u32>,
    pub skilltree_prefab_path: String,
    #[serde(rename = "UIAvatarModelPath")]
    pub uiavatar_model_path: String,
    pub ultra_skill_cut_in_prefab_path: String,
    pub waiting_avatar_head_icon_path: String,
}

#[derive(Default, Deserialize)]
pub struct TextMapHashEntry {
    pub hash: i64,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct FixPoint {
    pub raw_value: f64,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ItemEntry {
    #[serde(rename = "ItemID")]
    item_id: u32,
    item_num: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarSkillTreeConfig {
    pub anchor: String,
    pub pre_point: Vec<u32>,
    pub status_add_list: Vec<AvatarPropertyValue>,
    pub material_list: Vec<ItemEntry>,
    #[serde(rename = "LevelUpSkillID")]
    pub level_up_skill_id: Vec<u32>,
    pub icon_path: String,
    pub point_name: String,
    pub point_desc: String,
    pub ability_name: String,
    pub param_list: Vec<FixPoint>,
    #[serde(rename = "PointID")]
    pub point_id: u32,
    pub level: u32,
    #[serde(rename = "AvatarID")]
    pub avatar_id: u32,
    pub point_type: u32,
    pub max_level: u32,
    pub avatar_level_limit: u32,
    pub avatar_promotion_limit: u32,
    pub recommend_priority: u32,
    pub point_trigger_key: StringHash,
    pub default_unlock: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarPropertyValue {
    pub value: FixPoint,
    pub property_type: AvatarPropertyType,
}

#[derive(Default, Deserialize)]
pub enum AvatarPropertyType {
    #[default]
    Unknown = 0,
    MaxHP = 1,
    Attack = 2,
    Defence = 3,
    Speed = 4,
    CriticalChance = 5,
    CriticalDamage = 6,
    HealRatio = 7,
    StanceBreakAddedRatio = 8,
    SPRatio = 9,
    StatusProbability = 10,
    StatusResistance = 11,
    PhysicalAddedRatio = 12,
    PhysicalResistance = 13,
    FireAddedRatio = 14,
    FireResistance = 15,
    IceAddedRatio = 16,
    IceResistance = 17,
    ThunderAddedRatio = 18,
    ThunderResistance = 19,
    WindAddedRatio = 20,
    WindResistance = 21,
    QuantumAddedRatio = 22,
    QuantumResistance = 23,
    ImaginaryAddedRatio = 24,
    ImaginaryResistance = 25,
    BaseHP = 26,
    HPDelta = 27,
    BaseAttack = 28,
    AttackDelta = 29,
    BaseDefence = 30,
    DefenceDelta = 31,
    HPAddedRatio = 32,
    AttackAddedRatio = 33,
    DefenceAddedRatio = 34,
    BaseSpeed = 35,
    HealTakenRatio = 36,
    PhysicalResistanceDelta = 37,
    FireResistanceDelta = 38,
    IceResistanceDelta = 39,
    ThunderResistanceDelta = 40,
    WindResistanceDelta = 41,
    QuantumResistanceDelta = 42,
    ImaginaryResistanceDelta = 43,
    AllDamageReduce = 44,
    RelicValueExtraAdditionRatio = 45,
    EquipValueExtraAdditionRatio = 46,
    EquipExtraRank = 47,
    AvatarExtraRank = 48,
    AllDamageTypeAddedRatio = 49,
    SpeedAddedRatio = 50,
    SpeedDelta = 51,
    CriticalChanceBase = 52,
    CriticalDamageBase = 53,
    SPRatioBase = 54,
    HealRatioBase = 55,
    StatusProbabilityBase = 56,
    StatusResistanceBase = 57,
    BreakDamageAddedRatio = 58,
    BreakDamageAddedRatioBase = 59,
    MaxSP = 60,
    Count = 61,
    SpecialMaxSP = 62,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StringHash {
    pub hash: i64,
    pub is_empty: bool,
    pub hash_key: String,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TutorialConfig {
    pub tutorial_json_path: String,
    pub trigger_params: Vec<TriggerParam>,
    pub finish_trigger_params: Vec<TriggerParam>,
    #[serde(rename = "TutorialID")]
    pub tutorial_id: u32,
    pub priority: u32,
    pub can_interrupt: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TutorialGuideGroupConfig {
    #[serde(rename = "TutorialGuideIDList")]
    pub tutorial_guide_id_list: Vec<u32>,
    pub trigger_params: Vec<TriggerParam>,
    pub finish_trigger_params: Vec<TriggerParam>,
    #[serde(rename = "GroupID")]
    pub group_id: u32,
    pub tutorial_type: u32,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    pub message_text: TextMapHashEntry,
    pub can_review: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TriggerParam {
    pub trigger_param: String,
    pub trigger_type: String,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ItemConfig {
    pub custom_data_list: Vec<u32>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub inventory_display_tag: u32,
    pub is_visible: bool,
    pub item_avatar_icon_path: String,
    #[serde(rename = "ItemBGDesc")]
    pub item_bgdesc: i64,
    pub item_currency_icon_path: String,
    pub item_desc: i64,
    pub item_figure_icon_path: String,
    pub item_icon_path: String,
    pub item_main_type: String,
    pub item_name: i64,
    pub pile_limit: u32,
    pub purpose_type: u32,
    pub rarity: String,
    pub return_item_id_list: Vec<u32>,
    pub use_data_id: u32,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentConfig {
    pub avatar_base_type: String,
    pub avatar_detail_offset: Vec<f64>,
    pub battle_dialog_offset: Vec<f64>,
    pub coin_cost: u32,
    pub equipment_desc: TextMapHashEntry,
    #[serde(rename = "EquipmentID")]
    pub equipment_id: u32,
    pub equipment_name: TextMapHashEntry,
    pub exp_provide: u32,
    pub exp_type: u32,
    pub gacha_result_offset: Vec<f64>,
    pub image_path: String,
    pub item_right_panel_offset: Vec<f64>,
    pub max_promotion: u32,
    pub max_rank: u32,
    pub rank_up_cost_list: Vec<u32>,
    pub rarity: String,
    pub release: bool,
    #[serde(rename = "SkillID")]
    pub skill_id: u32,
    pub thumbnail_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RelicConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "SetID")]
    pub set_id: u32,
    #[serde(rename = "Type")]
    pub relic_type: RelicType,
    pub rarity: String,
    pub main_affix_group: u32,
    pub sub_affix_group: u32,
    pub max_level: u32,
    pub exp_type: u32,
    pub exp_provide: i32,
    pub coin_cost: u32,
    pub mode: String,
}

#[derive(Deserialize)]
pub enum RelicType {
    HEAD,
    NECK,
    HAND,
    BODY,
    FOOT,
    OBJECT,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MapEntranceConfig {
    #[serde(rename = "BeginMainMissionList")]
    pub begin_main_mission_idlist: Vec<u32>,
    #[serde(rename = "FinishMainMissionList")]
    pub finish_main_mission_idlist: Vec<u32>,
    #[serde(rename = "FinishSubMissionList")]
    pub finish_sub_mission_idlist: Vec<u32>,
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    #[serde(rename = "EntranceType")]
    pub entrance_type: String,
}
