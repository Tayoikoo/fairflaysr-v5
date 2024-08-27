use std::sync::Arc;

use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use proto::PlayerBasicInfo;
use proto::PlayerDataBin;
use tokio::sync::OnceCell;
use std::sync::Mutex;

use crate::database;
use crate::util;

use super::managers::*;
use super::PlayerInfo;

#[derive(Clone)]
pub struct GameContext {
    logged_in: OnceCell<()>,
    pub player: Arc<AtomicRefCell<PlayerInfo>>,
    pub avatar_mgr: Arc<AtomicRefCell<AvatarManager>>,
    // pub hero_basic_type_mgr: Arc<AtomicRefCell<HeroBasicTypeManager>>,
    pub multi_path_avatar_mgr: Arc<AtomicRefCell<HeroBasicTypeManager>>,
    pub item_mgr: Arc<AtomicRefCell<ItemManager>>,
    pub lineup_mgr: Arc<AtomicRefCell<LineupManager>>,
    pub scene_mgr: Arc<AtomicRefCell<SceneManager>>,
    pub time_mgr: Arc<AtomicRefCell<TimeManager>>,
    pub tutorial_mgr: Arc<AtomicRefCell<TutorialManager>>,
    pub monster_level: Arc<Mutex<u32>>,
    pub cycle_count: Arc<Mutex<u32>>,
    pub stage_id: Arc<Mutex<u32>>,
    pub energy: Arc<Mutex<u32>>,
}

impl GameContext {
    const SAVE_INTERVAL: u64 = 60;

    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        let item_mgr = Arc::new(AtomicRefCell::new(ItemManager::new(player.clone())));
        let avatar_mgr = Arc::new(AtomicRefCell::new(AvatarManager::new(
            player.clone(),
            item_mgr.clone(),
        )));
        let monster_level = Arc::new(Mutex::new(95));
        let cycle_count = Arc::new(Mutex::new(30));
        let stage_id = Arc::new(Mutex::new(201012311));
        let energy = Arc::new(Mutex::new(10000));
        Self {
            logged_in: OnceCell::new(),
            player: player.clone(),
            avatar_mgr,
            // hero_basic_type_mgr: Arc::new(AtomicRefCell::new(HeroBasicTypeManager::new(
            //     player.clone(),
            // ))),
            multi_path_avatar_mgr: Arc::new(AtomicRefCell::new(HeroBasicTypeManager::new(
                player.clone(),
            ))),            
            item_mgr,
            // Some variable to store for commands
            monster_level,
            cycle_count,
            stage_id,
            energy,            
            //
            lineup_mgr: Arc::new(AtomicRefCell::new(LineupManager::new(player.clone()))),
            scene_mgr: Arc::new(AtomicRefCell::new(SceneManager::new(player.clone()))),
            time_mgr: Arc::new(AtomicRefCell::new(TimeManager::new())),
            tutorial_mgr: Arc::new(AtomicRefCell::new(TutorialManager::new(player))),
        }
    }

    pub fn is_new_player(&self) -> bool {
        self.player.borrow().data.basic_bin.is_none()
    }

    pub fn is_logged_in(&self) -> bool {
        self.logged_in.get().is_some()
    }

    pub fn on_player_get_token_succ(&self, uid: u32, player_bin: PlayerDataBin) {
        let mut player_info = self.player.borrow_mut();

        player_info.uid = uid;
        player_info.data = player_bin;
    }

    pub fn get_uid(&self) -> u32 {
        let player_info = self.player.borrow();
        player_info.uid
    }

    pub fn is_player(&self) -> bool{
        let uid = self.get_uid();
        uid !=0
    }

    pub async fn on_player_logged_in(&self) -> Result<()> {
        self.logged_in.set(())?;

        let mut scene_mgr = self.scene_mgr.borrow_mut();
        let entry_id = scene_mgr.cur_entry_id();
        scene_mgr.enter_scene(entry_id).unwrap();

        let mut player_info = self.player.borrow_mut();
        let basic_comp = player_info.data.basic_bin.as_mut().unwrap();

        basic_comp.login_times += 1;
        basic_comp.last_login_timestamp = util::cur_timestamp_ms() as i64;

        database::update_player_bin(player_info.uid, &player_info.data).await
    }

    pub async fn on_player_heartbeat(&self, client_time_ms: u64) -> Result<()> {
        let seconds_since_last_save = self.time_mgr.borrow().seconds_since_last_save();
        let mut time_mgr = self.time_mgr.borrow_mut();

        if self.is_logged_in() && seconds_since_last_save > Self::SAVE_INTERVAL {
            let player_info = self.player.borrow();
            database::update_player_bin(player_info.uid, &player_info.data).await?;

            time_mgr.on_player_save();
        }

        time_mgr.update(client_time_ms);
        Ok(())
    }

    pub async fn on_player_logout(&self) -> Result<()> {
        if self.is_logged_in() {
            let player_info = self.player.borrow();
            database::update_player_bin(player_info.uid, &player_info.data).await
        } else {
            Ok(())
        }
    }

    pub fn player_basic_info_proto(&self) -> PlayerBasicInfo {
        let player_info = self.player.borrow();
        let basic_comp = player_info.data.basic_bin.as_ref().unwrap();

        PlayerBasicInfo {
            nickname: basic_comp.nickname.clone(),
            level: basic_comp.level,
            exp: basic_comp.exp,
            stamina: 240,
            ..Default::default()
        }
    }

    pub fn init_default_player(&self) {
        self.init_player_data();
        // self.hero_basic_type_mgr.borrow().init_defaults();
        self.multi_path_avatar_mgr.borrow().init_defaults();
        self.avatar_mgr.borrow().init_defaults();
        self.lineup_mgr.borrow().init_defaults();
        self.item_mgr.borrow().init_defaults();
        self.scene_mgr.borrow().init_defaults();
        self.tutorial_mgr.borrow().init_defaults();
    }

    fn init_player_data(&self) {
        self.player.borrow_mut().init_player_data();
    }
}
