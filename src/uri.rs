#![allow(dead_code)]


use reqwest;

use serde_derive::Deserialize;

// these item structs should be namespaced into the v2 api. 
#[derive(Deserialize, Debug)]
struct ItemAttribute {
    attribute: String,
    modifier: u32
}

#[derive(Deserialize, Debug)]
struct ItemDetails {
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
    details: ItemDetails,
}

// should be usize?

// todo: move this to some types thing?
pub struct ItemId(pub u128);
pub struct ItemStatId(pub u128);
pub struct RecipeId(pub u128);
pub struct ApiVersion(pub u8);
pub struct ApiKey(pub String);
pub struct AchievementId(pub u128);


#[allow(non_camel_case_types)]
pub enum EndPoint {
    achievements(Option<AchievementId>),
    achievements_daily,
    achievements_daily_tomorrow,
    account,
    account_achievements,
    account_bank,
    account_materials,
    account_dailycrafting,
    account_dungeons,
    account_dyes,
    items(ItemId),
    item_stats(Option<ItemId>),
    // this is nothing to do with a specific item.
    // item stats supporst 3 end points/
    // base /itemstats
    // id   /itemstats/id
    // multiple(?) /itemstats?ids=23
    item_stats_all(Option<ItemStatId>),
    recipes(Option<RecipeId>),
    build,
}

impl EndPoint {
    pub fn requires_auth(self) -> bool {
        match self {
            // do require auth.
	    EndPoint::account => true,
	    EndPoint::account_achievements => true,
            EndPoint::account_bank => true,
            EndPoint::account_materials => true,
	    EndPoint::account_dailycrafting => true,
	    EndPoint::account_dungeons => true,
	    EndPoint::account_dyes => true,

            // don't require auth
	    EndPoint::achievements(_) => false,
	    EndPoint::achievements_daily => false,
	    EndPoint::achievements_daily_tomorrow => false,
            EndPoint::items(_) => false,
            EndPoint::item_stats(_) => false,
            EndPoint::item_stats_all(_) => false,
            EndPoint::recipes(_) => false,
            EndPoint::build => false,
        }
    }

    pub fn uri(&self) -> String {
        match self {
	    EndPoint::account => format!("account"),
	    EndPoint::account_achievements => format!("account/achievements"),
	    EndPoint::account_dailycrafting => format!("account/dailycrafting"),
	    EndPoint::account_dungeons => format!("account/dungeons"),
	    EndPoint::account_dyes => format!("account/dyes"),
	    EndPoint::achievements(op_id) => {
		match op_id {
		    Some(id) => format!("achievements/{}", id.0.to_string()),
		    None => format!("achievements"),
		}
	    },
	    EndPoint::achievements_daily => "achievements/daily".to_string(),
	    EndPoint::achievements_daily_tomorrow => "achievements/daily/tomorrow".to_string(),
            EndPoint::account_materials => "account/materials".to_string(),
            EndPoint::account_bank => "account/bank".to_string(),
            EndPoint::items(id) => format!("items/{}", id.0.to_string()),
	    EndPoint::item_stats(op_stats_id) => {
		match op_stats_id {
		    Some(stats_id) => format!("itemstats/{}", stats_id.0.to_string()),
		    None => "itemstats".to_string(),
		}
	    },
	    EndPoint::item_stats_all(op_item_stat_id) => {
		match op_item_stat_id {
		    Some(item_stat_id) => format!("itemstats/{}", item_stat_id.0.to_string()),
		    None => "itemstats".to_string(),
		}
	    },
	    EndPoint::recipes(op_recipe_id) => {
		match op_recipe_id {
		    Some(recipe_id) =>  format!("recipes/{}", recipe_id.0.to_string()),
		    None => "recipes".to_string(),
		}
	    },
	    EndPoint::build => "build".to_string(),
        }
    }
}

pub struct Requester {
    version: ApiVersion,
    api_key: Option<ApiKey>,
    base_uri: String,
}

impl Requester {
    // todo: add in auth key as input parameter. 
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

    #[test]
    fn test_item_construction() {
        let p = EndPoint::items(ItemId(3));
        assert_eq!(p.uri(), "items/3");
        let k = EndPoint::items(ItemId(1000));
        assert_eq!(k.uri(), "items/1000");
    }

    #[test]
    fn test_uri_achivements() {
	let p = EndPoint::achievements(None);
	assert_eq!(p.uri(), "achievements");
    }

    #[test]
    fn test_uri_achivement_ids() {
	let p = EndPoint::achievements(Some(AchievementId(32)));
	assert_eq!(p.uri(), "achievements/32");
    }

    // need to wait till can combine specific end points. 
    fn test_uri_achive_builder() {
	// https://wiki.guildwars2.com/wiki/API:2/achievements
	let p = EndPoint::achievements(Some(AchievementId(32)));
	let k = EndPoint::achievements(Some(AchievementId(40)));

	// would like to use the data refinement thing here to specify
	//  a specific varient of the EndPoint enum item at compile time.
	// it appears to not be implements
	// https://github.com/rust-lang/rfcs/issues/754
	// pretty certain order of ids in result do not matter. 
	fn mock_builder(end_point1: &EndPoint,
			end_point2: &EndPoint) -> String {
	    let id_one = match end_point1 {
		EndPoint::achievements(t) => {
		    match t {
			Some(id) => id,
			None => { panic!("shouldn't get here") },
		    }
		},
		_ => { panic!("shouldn't get here") },
	    };
	    let id_two = match end_point2 {
		EndPoint::achievements(t) => {
		    match t {
			Some(id) => id,
			None => { panic!("shouldn't get here") },
		    }
		},
		_ => { panic!("shouldn't get here") },
	    };
	    // should return  "achivements?ids=id_one,id_two"
	    // specifically fails cause its not implemented yet. 
	    return "no implemented".to_string();
	}
	assert_eq!(mock_builder(&p, &k), "achievements?ids=32,40")
    }
    
    #[test]
    fn uri_building() {
        let r = Requester::new(ApiVersion(2), None);
        let result = r.build_uri(&EndPoint::account_bank);
        assert_eq!(result, "https://api.guildwars2.com/v2/account/bank");
    }

    #[test]
    fn uri_query() {
        let requester = Requester::new(ApiVersion(2), None);

	let r = reqwest::blocking::Client::new()
	    .get(&requester.build_uri(&EndPoint::items(ItemId(2000))))
	    .send().unwrap().text().unwrap();
	println!("{}", r);

	let k: Item = reqwest::blocking::Client::new()
	    .get(&requester.build_uri(&EndPoint::items(ItemId(2000))))
	    .send().unwrap().json().unwrap();
	// todo: what to assert here. 
    }
}
