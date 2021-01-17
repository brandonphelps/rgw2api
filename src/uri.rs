use serde_derive::Deserialize;

// these item structs should be namespaced into the v2 api.
// todo: move all these types into some common space.
#[derive(Deserialize, Debug)]
struct ItemAttribute {
    attribute: String,
    modifier: u32,
}

#[derive(Deserialize, Debug)]
struct ArmorDetails {}

#[derive(Deserialize, Debug)]
struct WeaponDetails {}

#[derive(Deserialize, Debug)]
enum ItemDetailsT {
    Armor(ArmorDetails),
    Weapon(WeaponDetails),
}

#[derive(Deserialize, Debug)]
struct ItemDetails {
    // todo: can't use type directly, what should this attribe be named?
    #[serde(rename = "type")]
    c_type: String,
    weight_class: String,
    defense: u32,
    // whats the type of infusions slots item?
    //infusion_slots:
    //infix_upgrade : HashMap<String,
    // infix upgrade is of format { "id" : u32, "attributes" : Vec<ItemAttribute> }
    // bit un certain how to tell serdea the values are of different types https://github.com/serde-rs/json/issues/144
    attribute_adjustment: f64,
    suffix_item_id: u64,
    secondary_suffix_item_id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    // maybe should be an option?
    description: String,
    #[serde(rename = "type")]
    item_type: String,
    id: u32,
    level: u32,
    rarity: String,
    default_skin: u64,
    game_types: Vec<String>,
    flags: Vec<String>,
    // not certain what the vec type should be.
    restrictions: Vec<String>,
    chat_link: String,
    icon: String,
    details: ItemDetailsT,
}

// should be usize?

// todo: move this to some types thing?
pub struct ItemId(pub u32);
pub struct ItemStatId(pub u32);
pub struct RecipeId(pub u32);
pub struct ApiVersion(pub u8);
pub struct ApiKey(pub String);
pub struct AchievementId(pub u32);

// List of end points that are accessible from the guild wars 2 api.
// Each endpoint indicates if authentication is required to access the contents.
// An endpoint looks like EndPoint::Items(None) which then maps to -> <base_url>/items
pub enum EndPoint {
    Achievements(Option<AchievementId>),
    AchievementsDaily,
    AchievementsDailyTomorrow,
    Account,
    AccountAchievements,
    AccountBank,
    AccountMaterials,
    AccountDailycrafting,
    AccountDungeons,
    AccountDyes,
    Items(Option<ItemId>),
    ItemStats(Option<ItemId>),
    // this is nothing to do with a specific item.
    // item stats supporst 3 end points/
    // base /itemstats
    // id   /itemstats/id
    // multiple(?) /itemstats?ids=23
    ItemStatsAll(Option<ItemStatId>),
    Recipes(Option<RecipeId>),
    Build,
}

impl EndPoint {
    pub fn requires_auth(self) -> bool {
        match self {
            // do require auth.
            EndPoint::Account => true,
            EndPoint::AccountAchievements => true,
            EndPoint::AccountBank => true,
            EndPoint::AccountMaterials => true,
            EndPoint::AccountDailycrafting => true,
            EndPoint::AccountDungeons => true,
            EndPoint::AccountDyes => true,

            // don't require auth
            EndPoint::Achievements(_) => false,
            EndPoint::AchievementsDaily => false,
            EndPoint::AchievementsDailyTomorrow => false,
            EndPoint::Items(_) => false,
            EndPoint::ItemStats(_) => false,
            EndPoint::ItemStatsAll(_) => false,
            EndPoint::Recipes(_) => false,
            EndPoint::Build => false,
        }
    }

    pub fn uri(&self) -> String {
        match self {
            EndPoint::Account => format!("account"),
            EndPoint::AccountAchievements => format!("account/achievements"),
            EndPoint::AccountDailycrafting => format!("account/dailycrafting"),
            EndPoint::AccountDungeons => format!("account/dungeons"),
            EndPoint::AccountDyes => format!("account/dyes"),
            EndPoint::Achievements(op_id) => match op_id {
                Some(id) => format!("achievements/{}", id.0.to_string()),
                None => format!("achievements"),
            },
            EndPoint::AchievementsDaily => "achievements/daily".to_string(),
            EndPoint::AchievementsDailyTomorrow => "achievements/daily/tomorrow".to_string(),
            EndPoint::AccountMaterials => "account/materials".to_string(),
            EndPoint::AccountBank => "account/bank".to_string(),
            EndPoint::Items(op_id) => match op_id {
                Some(id) => {
                    format!("items/{}", id.0.to_string())
                }
                None => "items".to_string(),
            },
            EndPoint::ItemStats(op_stats_id) => match op_stats_id {
                Some(stats_id) => format!("itemstats/{}", stats_id.0.to_string()),
                None => "itemstats".to_string(),
            },
            EndPoint::ItemStatsAll(op_item_stat_id) => match op_item_stat_id {
                Some(item_stat_id) => format!("itemstats/{}", item_stat_id.0.to_string()),
                None => "itemstats".to_string(),
            },
            EndPoint::Recipes(op_recipe_id) => match op_recipe_id {
                Some(recipe_id) => format!("recipes/{}", recipe_id.0.to_string()),
                None => "recipes".to_string(),
            },
            EndPoint::Build => "build".to_string(),
        }
    }
}

// todo: likely should be renamed to EndPointBuilder?
// can let something else do the requesting?
pub struct Requester {
    // todo: the version here, would likely need to be apart of the endpoints since
    // the uri and or other properties could change if version was modified.
    version: ApiVersion,
    api_key: Option<ApiKey>,
    base_uri: String,
}

impl Requester {
    pub fn new(version: ApiVersion, api_key: Option<ApiKey>) -> Requester {
        let mut uri_str = String::new();
        uri_str += "https://api.guildwars2.com/v";
        uri_str += &version.0.to_string();
        return Requester {
            version: version,
            api_key: api_key,
            base_uri: uri_str,
        };
    }

    pub fn build_uri(&self, end_point: &EndPoint) -> String {
        let mut new_uri = self.base_uri.clone();
        new_uri.push_str("/");
        new_uri.push_str(&end_point.uri().clone());
        return new_uri;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use reqwest;

    #[test]
    fn test_item_construction() {
        let p = EndPoint::Items(Some(ItemId(3)));
        assert_eq!(p.uri(), "items/3");
        let k = EndPoint::Items(Some(ItemId(1000)));
        assert_eq!(k.uri(), "items/1000");
    }

    #[test]
    fn test_item_none() {
        let p = EndPoint::Items(None);
        assert_eq!(p.uri(), "items");
    }

    #[test]
    fn test_uri_achivements() {
        let p = EndPoint::Achievements(None);
        assert_eq!(p.uri(), "achievements");
    }

    #[test]
    fn test_uri_achivement_ids() {
        let p = EndPoint::Achievements(Some(AchievementId(32)));
        assert_eq!(p.uri(), "achievements/32");
    }

    // need to wait till can combine specific end points.
    fn test_uri_achive_builder() {
        // https://wiki.guildwars2.com/wiki/API:2/achievements
        let p = EndPoint::Achievements(Some(AchievementId(32)));
        let k = EndPoint::Achievements(Some(AchievementId(40)));

        // would like to use the data refinement thing here to specify
        //  a specific varient of the EndPoint enum item at compile time.
        // it appears to not be implements
        // https://github.com/rust-lang/rfcs/issues/754
        // pretty certain order of ids in result do not matter.
        fn mock_builder(end_point1: &EndPoint, end_point2: &EndPoint) -> String {
            let _id_one = match end_point1 {
                EndPoint::Achievements(t) => match t {
                    Some(id) => id,
                    None => {
                        panic!("shouldn't get here")
                    }
                },
                _ => {
                    panic!("shouldn't get here")
                }
            };
            let _id_two = match end_point2 {
                EndPoint::Achievements(t) => match t {
                    Some(id) => id,
                    None => {
                        panic!("shouldn't get here")
                    }
                },
                _ => {
                    panic!("shouldn't get here")
                }
            };
            // should return  "achivements?ids=id_one,id_two"
            // specifically fails cause its not implemented yet.
            return "no implemented".to_string();
        }
        assert_eq!(mock_builder(&p, &k), "achievements?ids=32,40")
    }

    #[test]
    fn test_uri_account_achivements() {
        let p = EndPoint::AccountAchievements;
        assert_eq!(p.requires_auth(), true);
    }

    #[test]
    fn uri_building() {
        let r = Requester::new(ApiVersion(2), None);
        let result = r.build_uri(&EndPoint::AccountBank);
        assert_eq!(result, "https://api.guildwars2.com/v2/account/bank");
    }

    #[test]
    fn uri_query() {
        let requester = Requester::new(ApiVersion(2), None);

        let r = reqwest::blocking::Client::new()
            // todo: having to wrap each option item for the end point as Some is really cumbersome,
            // kinda looks like lisp with all the functional parans.
            .get(&requester.build_uri(&EndPoint::Items(Some(ItemId(2000)))))
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("{}", r);

        let _k: Item = reqwest::blocking::Client::new()
            .get(&requester.build_uri(&EndPoint::Items(Some(ItemId(2000)))))
            .send()
            .unwrap()
            .json()
            .unwrap();
        // todo: what to assert here.
    }
}
