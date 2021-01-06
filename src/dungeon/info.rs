//! Contains information about dungeons that are not avaliable from the GW2 API.
//! This information would have to be updated if dungeon rewards are reworked
//! or if more dungeons are added.
//!
//! TODO: The amount of gold rewards are known for some story paths.
//!       The missing story paths are currently using 0.

/// Information about a dungeon.
#[derive(Debug, PartialEq, Eq)]
pub struct DungeonInfo {
    /// The id used by the /v2/dungeons endpoint.
    pub id: &'static str,
    /// A user friendly short name.
    pub short_name: &'static str,
    /// A user friendly long name.
    pub long_name: &'static str,
    /// id of the achievement for collection all skins.
    pub collection_id: u32,
    /// id of the token used by the /v2/account/currencies endpoint.
    pub currency_id: u32,
    /// information about every path in this dungeon.
    pub paths: &'static [PathInfo],
}

/// Information about a path within a dungeon.
#[derive(Debug, PartialEq, Eq)]
pub struct PathInfo {
    /// The id used by the /v2/account/dungeons endpoint to indicate
    /// if a user has done this path today.
    pub id: &'static str,
    /// A user friendly short name.
    pub short_name: &'static str,
    /// A longer user friendly name.
    pub long_name: &'static str,
    /// The index inserted into the bits array for the dungeon
    /// frequenter achievement when this path is done.
    pub dungeon_frequenter_index: Option<u8>,
    /// The rewards for doing this path
    pub rewards: Rewards,
}

/// Describes the rewards for a path.
#[derive(Debug, PartialEq, Eq)]
pub enum Rewards {
    /// Story mission.  No token rewards.  Fixed coin reward.
    Story { coins: u32 },
    /// Explorable path.  100 tokens on first per day, 20 on repeat.
    /// bonus_coins + 26s on first per day.  26s on repeat.
    Explorable { bonus_coins: u32 },
}

static AC_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "ac_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(4),
        rewards: Rewards::Story { coins: 13_00 },
    },
    PathInfo {
        id: "hodgins",
        short_name: "p1",
        long_name: "Hodgins (p1)",
        dungeon_frequenter_index: Some(5),
        rewards: Rewards::Explorable { bonus_coins: 50_00 },
    },
    PathInfo {
        id: "detha",
        short_name: "p2",
        long_name: "Detha (p2)",
        dungeon_frequenter_index: Some(6),
        rewards: Rewards::Explorable { bonus_coins: 50_00 },
    },
    PathInfo {
        id: "tzark",
        short_name: "p3",
        long_name: "Tzark (p3)",
        dungeon_frequenter_index: Some(7),
        rewards: Rewards::Explorable { bonus_coins: 50_00 },
    },
];

static CM_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "cm_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(12),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "asura",
        short_name: "p1",
        long_name: "Asura (p1)",
        dungeon_frequenter_index: Some(13),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "seraph",
        short_name: "p2",
        long_name: "Seraph (p2)",
        dungeon_frequenter_index: Some(14),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "butler",
        short_name: "p3",
        long_name: "Butler (p3)",
        dungeon_frequenter_index: Some(15),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
];

static TA_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "ta_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(20),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "leurent",
        short_name: "up",
        long_name: "Leurent (Up)",
        dungeon_frequenter_index: Some(21),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "vevina",
        short_name: "forward",
        long_name: "Vevina (Forward)",
        dungeon_frequenter_index: Some(22),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "aetherpath",
        short_name: "aetherpath",
        long_name: "Aetherpath",
        dungeon_frequenter_index: Some(23),
        rewards: Rewards::Explorable { bonus_coins: 66_00 },
    },
];

static SE_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "se_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(16),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "fergg",
        short_name: "p1",
        long_name: "Fergg (p1)",
        dungeon_frequenter_index: Some(17),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "rasalov",
        short_name: "p2",
        long_name: "Rasolov (p2)",
        dungeon_frequenter_index: Some(18),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "koptev",
        short_name: "p3",
        long_name: "Koptev (p3)",
        dungeon_frequenter_index: Some(19),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
];

static COF_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "cof_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(28),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "ferrah",
        short_name: "p1",
        long_name: "Ferrah (p1)",
        dungeon_frequenter_index: Some(29),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "magg",
        short_name: "p2",
        long_name: "Magg (p2)",
        dungeon_frequenter_index: Some(30),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "rhiannon",
        short_name: "p3",
        long_name: "Rhiannon (p3)",
        dungeon_frequenter_index: Some(31),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
];

static HOTW_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "hotw_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(24),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "butcher",
        short_name: "p1",
        long_name: "Butcher (p1)",
        dungeon_frequenter_index: Some(25),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "plunderer",
        short_name: "p2",
        long_name: "Plunderer (p2)",
        dungeon_frequenter_index: Some(26),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "zealot",
        short_name: "p3",
        long_name: "Zealot (p3)",
        dungeon_frequenter_index: Some(27),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
];

static COE_PATHS: [PathInfo; 4] = [
    PathInfo {
        id: "coe_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: Some(0),
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "submarine",
        short_name: "p1",
        long_name: "Submarine (p1)",
        dungeon_frequenter_index: Some(1),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "teleporter",
        short_name: "p2",
        long_name: "Teleporter (p2)",
        dungeon_frequenter_index: Some(2),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
    PathInfo {
        id: "front_door",
        short_name: "p3",
        long_name: "Front Door (p3)",
        dungeon_frequenter_index: Some(3),
        rewards: Rewards::Explorable { bonus_coins: 35_00 },
    },
];

static ARAH_PATHS: [PathInfo; 5] = [
    PathInfo {
        id: "arah_story",
        short_name: "story",
        long_name: "Story",
        dungeon_frequenter_index: None,
        rewards: Rewards::Story { coins: 0 },
    },
    PathInfo {
        id: "jotun",
        short_name: "p1",
        long_name: "Jotun (p1)",
        dungeon_frequenter_index: Some(8),
        rewards: Rewards::Explorable {
            bonus_coins: 1_80_00,
        },
    },
    PathInfo {
        id: "mursaat",
        short_name: "p2",
        long_name: "Mursaat (p2)",
        dungeon_frequenter_index: Some(9),
        rewards: Rewards::Explorable {
            bonus_coins: 1_05_00,
        },
    },
    PathInfo {
        id: "forgotten",
        short_name: "p3",
        long_name: "Forgotten (p3)",
        dungeon_frequenter_index: Some(10),
        rewards: Rewards::Explorable { bonus_coins: 50_00 },
    },
    PathInfo {
        id: "seer",
        short_name: "p4",
        long_name: "Seer (p4)",
        dungeon_frequenter_index: Some(11),
        rewards: Rewards::Explorable {
            bonus_coins: 1_80_00,
        },
    },
];

pub static DUNGEONS: [DungeonInfo; 8] = [
    DungeonInfo {
        id: "ascalonian_catacombs",
        short_name: "ac",
        long_name: "Ascalonian Catacombs",
        collection_id: 1725,
        currency_id: 5,
        paths: &AC_PATHS,
    },
    DungeonInfo {
        id: "caudecus_manor",
        currency_id: 9,
        short_name: "cm",
        long_name: "Caudecus's Manor",
        collection_id: 1723,
        paths: &CM_PATHS,
    },
    DungeonInfo {
        id: "twilight_arbor",
        short_name: "ta",
        long_name: "Twilight Arbor",
        collection_id: 1721,
        currency_id: 11,
        paths: &TA_PATHS,
    },
    DungeonInfo {
        id: "sorrows_embrace",
        short_name: "se",
        long_name: "Sorrow's Embrace",
        collection_id: 1722,
        currency_id: 10,
        paths: &SE_PATHS,
    },
    DungeonInfo {
        id: "citadel_of_flame",
        short_name: "cof",
        long_name: "Citadel of Flame",
        collection_id: 1714,
        currency_id: 13,
        paths: &COF_PATHS,
    },
    DungeonInfo {
        id: "honor_of_the_waves",
        short_name: "hotw",
        long_name: "Honor of the Waves",
        collection_id: 1718,
        currency_id: 12,
        paths: &HOTW_PATHS,
    },
    DungeonInfo {
        id: "crucible_of_eternity",
        short_name: "coe",
        long_name: "Crucible of Eternity",
        collection_id: 1719,
        currency_id: 14,
        paths: &COE_PATHS,
    },
    DungeonInfo {
        id: "ruined_city_of_arah",
        short_name: "arah",
        long_name: "The Ruined City of Arah",
        collection_id: 1724,
        currency_id: 6,
        paths: &ARAH_PATHS,
    },
];
