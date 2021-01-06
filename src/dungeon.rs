mod info;
use info::{DungeonInfo, PathInfo, Rewards, DUNGEONS};

mod user;
pub use user::UserProgress;

/// Information about a dungeon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dungeon {
    info: &'static DungeonInfo,
}

/// Information about a path within a dungeon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    info: &'static PathInfo,
    dungeon: &'static DungeonInfo,
}

impl Dungeon {
    /// Gets every dungeon.
    pub fn all() -> Vec<Self> {
        DUNGEONS.iter().map(|x| Self { info: x }).collect()
    }

    /// Gets the name of the dungeon.
    pub fn name(&self) -> &str {
        self.info.long_name
    }

    /// Gets a short name for this dungeon.
    pub fn short_name(&self) -> &str {
        self.info.short_name
    }

    /// Gets the achievement id of the skin collection achievement for this dungeon.
    pub fn collection_id(&self) -> u32 {
        self.info.collection_id
    }

    /// Gets the wallet currency id of the dungeon token for this dungeon.
    pub fn currency_id(&self) -> u32 {
        self.info.currency_id
    }

    /// Gets all paths in this dungeon.
    pub fn paths(&self) -> Vec<Path> {
        self.info
            .paths
            .iter()
            .map(|path| Path {
                info: path,
                dungeon: self.info,
            })
            .collect()
    }
}

impl Path {
    /// Gets every dungeon path.
    pub fn all() -> Vec<Path> {
        Dungeon::all().iter().flat_map(Dungeon::paths).collect()
    }

    /// Looks up a path by its id.
    pub fn from_id(path_id: &str) -> Option<Path> {
        for path in Self::all() {
            if path_id == path.id() {
                return Some(path);
            }
        }
        None
    }

    /// Looks up a path by its index into the dungeon frequenter bits list.
    pub fn from_dungeon_frequenter_index(index: u8) -> Option<Path> {
        for path in Self::all() {
            if Some(index) == path.dungeon_frequenter_index() {
                return Some(path);
            }
        }
        None
    }

    /// Gets the dungeon that contains this path.
    pub fn dungeon(&self) -> Dungeon {
        Dungeon { info: self.dungeon }
    }

    /// Gets the unique dungeon path id used by the GW2 API for this path.
    pub fn id(&self) -> &str {
        self.info.id
    }

    /// Gets the name of this path.
    pub fn name(&self) -> &str {
        self.info.long_name
    }

    /// Gets a short name for this path. This is typically used in LFG.
    pub fn short_name(&self) -> &str {
        self.info.short_name
    }

    /// Gets the index into the dungeon frequenter achievement bits array
    /// in the GW2 achievement API for this dungeon path.
    pub fn dungeon_frequenter_index(&self) -> Option<u8> {
        self.info.dungeon_frequenter_index
    }

    /// The number of coins gotten by doing this dungeon path the first
    /// time in a day.
    pub fn coins(&self) -> u32 {
        match self.info.rewards {
            Rewards::Story { coins } => coins,
            Rewards::Explorable { bonus_coins } => 26_00 + bonus_coins,
        }
    }

    /// The number of coins gotten by doing this dungeon repeatedly in a day.
    pub fn repeat_coins(&self) -> u32 {
        match self.info.rewards {
            Rewards::Story { coins } => coins,
            Rewards::Explorable { .. } => 26_00,
        }
    }

    /// The number of tokens gotten by doing this dungeon path the first
    /// time in a day.
    pub fn tokens(&self) -> u32 {
        match self.info.rewards {
            Rewards::Story { .. } => 0,
            Rewards::Explorable { .. } => 100,
        }
    }

    /// The number of tokens gotten by doing this dungeon repeatedly in a day.
    pub fn repeat_tokens(&self) -> u32 {
        match self.info.rewards {
            Rewards::Story { .. } => 0,
            Rewards::Explorable { .. } => 20,
        }
    }
}

#[cfg(test)]
/// NOTE: Many tests have hard coded constants that must be changed if dungeon rewards
/// are reworked or more dungeons are added.
mod test {
    use super::{Dungeon, Path};

    #[test]
    fn all_dungeons() {
        assert_eq!(Dungeon::all().len(), 8);
    }

    #[test]
    fn all_paths() {
        assert_eq!(Path::all().len(), 33);
    }

    #[test]
    fn from_id() {
        assert_eq!(Path::from_id("coe_story").unwrap().id(), "coe_story");
        assert!(Path::from_id("bad_id").is_none());
    }

    #[test]
    fn from_dungeon_frequenter_index() {
        assert_eq!(
            Path::from_dungeon_frequenter_index(5)
                .unwrap()
                .dungeon_frequenter_index(),
            Some(5)
        );
        assert!(Path::from_dungeon_frequenter_index(100).is_none());
    }

    #[test]
    fn ac_story_rewards() {
        let path = Path::from_id("ac_story").unwrap();

        assert_eq!(13_00, path.coins());
        assert_eq!(13_00, path.repeat_coins());

        assert_eq!(0, path.tokens());
        assert_eq!(0, path.repeat_tokens());
    }

    #[test]
    fn ac_p1_rewards() {
        let path = Path::from_id("hodgins").unwrap();

        assert_eq!(50_00 + 26_00, path.coins());
        assert_eq!(26_00, path.repeat_coins());

        assert_eq!(100, path.tokens());
        assert_eq!(20, path.repeat_tokens());
    }
}
