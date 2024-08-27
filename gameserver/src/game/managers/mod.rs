use super::PlayerInfo;
use atomic_refcell::AtomicRefCell;
use std::sync::{Arc, Weak};

mod avatar;
// mod hero_basic_type;
mod multi_path_avatar;
mod item;
mod lineup;
mod scene;
mod time;
mod tutorial;

pub use avatar::AvatarManager;
// pub use hero_basic_type::HeroBasicTypeManager;
pub use multi_path_avatar::HeroBasicTypeManager;
pub use item::ItemManager;
pub use lineup::LineupManager;
pub use scene::SceneManager;
pub use time::TimeManager;
pub use tutorial::TutorialManager;
