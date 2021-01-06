use super::Path;
use std::collections::HashSet;

/// Tracks what dungeons a user has ran since the last daily reset
/// and from the last Dungeon Frequenter achievement completion.
pub struct UserProgress {
    dungeon_frequenter_progress: HashSet<u8>,
    dungeons_ran_today: HashSet<String>,
}

impl UserProgress {
    /// Fetches a user's dungeon progress from the GW2 API.
    pub fn from_api_key(_key: &str) -> Self {
        todo!()
    }

    /// Modifies a player info as if the player ran a dungeon path.
    pub fn run_path(&mut self, path: &Path) {
        // handle dungeon frequenter achievement update
        if let Some(index) = path.dungeon_frequenter_index() {
            self.dungeon_frequenter_progress.insert(index);
            if self.dungeon_frequenter_progress.len() == 8 {
                self.dungeon_frequenter_progress.clear();
            }
        }

        // handle daily update
        self.dungeons_ran_today.insert(path.id().to_owned());
    }

    /// Modifies a player info as if daily reset happened.
    pub fn daily_reset(&mut self) {
        self.dungeons_ran_today.clear();
    }

    /// Gets if a dungeon path has been ran today.
    pub fn has_ran_today(&self, path: &Path) -> bool {
        self.dungeons_ran_today.contains(path.id())
    }

    /// Gets if a dungeon path gives dungeon frequenter credit if it was
    /// ran next.
    pub fn gives_dungeon_frequenter_credit(&self, path: &Path) -> bool {
        match path.dungeon_frequenter_index() {
            Some(index) => !self.dungeon_frequenter_progress.contains(&index),
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Create a new player info with no paths ever ran for testing.
    fn empty() -> UserProgress {
        UserProgress {
            dungeon_frequenter_progress: Default::default(),
            dungeons_ran_today: Default::default(),
        }
    }

    #[test]
    fn ran_none() {
        let info = empty();
        for path in Path::all() {
            // all paths that can give progress do
            assert_eq!(
                path.dungeon_frequenter_index().is_some(),
                info.gives_dungeon_frequenter_credit(&path),
                "path: {}",
                path.id()
            );
            // not ran any paths today
            assert!(!info.has_ran_today(&path), "path: {}", path.id());
        }
    }

    #[test]
    fn ran_one() {
        for path in Path::all() {
            let mut info = empty();
            info.run_path(&path);

            assert!(
                !info.gives_dungeon_frequenter_credit(&path),
                "path: {}",
                path.id()
            );
            assert!(info.has_ran_today(&path), "path: {}", path.id());
            if path.dungeon_frequenter_index().is_some() {
                assert_eq!(1, info.dungeon_frequenter_progress.len());
            } else {
                assert_eq!(0, info.dungeon_frequenter_progress.len());
            }
        }
    }

    #[test]
    fn ran_one_yesterday() {
        let mut info = empty();
        let ran_path = Path::from_id("ac_story").unwrap();
        info.run_path(&ran_path);
        info.daily_reset();

        assert!(!info.has_ran_today(&ran_path));
        assert!(!info.gives_dungeon_frequenter_credit(&ran_path));
    }

    #[test]
    fn frequenter_finished() {
        let mut info = empty();
        [
            "ac_story", "hodgins", "detha", "tzark", "cm_story", "asura", "seraph", "butler",
            "ta_story",
        ]
        .iter()
        .map(|id| Path::from_id(id).unwrap())
        .for_each(|path| info.run_path(&path));

        // dungeon frequenter should have completed right before ta_story,
        // so only ta_story should be in the dungeon frequenter set.
        assert_eq!(1, info.dungeon_frequenter_progress.len());
        assert!(info.dungeon_frequenter_progress.contains(
            &Path::from_id("ta_story")
                .unwrap()
                .dungeon_frequenter_index()
                .unwrap()
        ));
    }

    #[test]
    fn daily_reset() {
        let mut info = empty();
        let path = Path::from_id("ac_story").unwrap();
        info.run_path(&path);
        info.daily_reset();

        assert!(!info.has_ran_today(&path));
    }
}
