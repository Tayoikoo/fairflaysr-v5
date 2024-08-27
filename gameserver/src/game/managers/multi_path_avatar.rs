use proto::{MultiPathAvatarType, PlayerHeroPathCompBin, HeroPathTypeBin};
use crate::game::gameplay_config::INSTANCE;
use super::*;

pub struct HeroBasicTypeManager {
    player_info: Arc<AtomicRefCell<PlayerInfo>>,
}

impl HeroBasicTypeManager {
    pub fn new(player_info: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player_info }
    }

    pub fn init_defaults(&self) {
        let mut player_info = self.player_info.borrow_mut();
        let gameplay_conf = INSTANCE.lock().unwrap();

        let mc_path = gameplay_conf
            .multipath_config
            .avatar_type
            .iter()
            .find(|config| config.avatar_id == 8001)
            .map(|config| config.path.clone())
            .expect("Trailbazer Path not found in config!");

        let m7th_path = gameplay_conf
            .multipath_config
            .avatar_type
            .iter()
            .find(|config| config.avatar_id == 1001)
            .map(|config| config.path.clone())
            .expect("March 7Th Path not found in config!");

        let default_march_path = MultiPathAvatarType::from_str_name(&m7th_path)
        .unwrap_or_else(|| {
            // Log the error or take appropriate action
            println!("(ERROR) Invalid path name: {}", m7th_path);
            // Provide a default value
            MultiPathAvatarType::Mar7thRogueType // Replace with a valid default variant
        })
        .into();
        
        let default_mc_path = MultiPathAvatarType::from_str_name(&mc_path)
        .unwrap_or_else(|| {
            // Log the error or take appropriate action
            println!("(ERROR) Invalid path name: {}", mc_path);
            // Provide a default value
            MultiPathAvatarType::BoyShamanType // Replace with a valid default variant
        })
        .into();
        
        println!("(DEBUG) MC name: {}", mc_path);
        println!("(DEBUG) March name: {}", m7th_path);

        player_info.data.multipath_bin = Some(PlayerHeroPathCompBin {
            cur_mc_path: default_mc_path,
            cur_march_path: default_march_path,
            multipath_type_list: vec![
                HeroPathTypeBin {
                    basic_type: default_mc_path,
                    rank: 6,  // Replace with the actual value for mc_rank
                },
                HeroPathTypeBin {
                    basic_type: default_march_path,
                    rank: 6,  // Replace with the actual value for march_rank
                },
            ],     
        });
    }
}
