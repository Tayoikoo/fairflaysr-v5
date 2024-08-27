use proto::{Gender, HeroBasicType, HeroBasicTypeBin, PlayerHeroCompBin};

use crate::game::gameplay_conf;

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

        let default_basic_type = HeroBasicType::from_str_name(&gameplay_conf.hero_basic_type)
            .unwrap()
            .into();

        player_info.data.hero_bin = Some(PlayerHeroCompBin {
            gender: Gender::from_str_name(&gameplay_conf.hero_gender)
                .unwrap()
                .into(),
            cur_basic_type: default_basic_type,
            basic_type_list: vec![HeroBasicTypeBin {
                basic_type: default_basic_type,
                rank: 6,
            }],
        });
    }
}
