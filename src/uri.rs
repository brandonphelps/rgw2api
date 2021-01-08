#![allow(dead_code)]

// should be usize?

// todo: move this to some types thing?
pub struct ItemId(pub u128);
pub struct RecipeId(pub u128);
pub struct ApiVersion(pub u8);

#[allow(non_camel_case_types)]
pub enum EndPoint {
    account_materials,
    account_bank,
    items(ItemId),
    item_stats(ItemId),
    item_stats_all, // this is nothing to do with a specific item.
    recipes(RecipeId),
    build,
}

impl EndPoint {
    pub fn requires_auth(self) -> bool {
        match self {
            // do require auth.
            EndPoint::account_materials => true,
            EndPoint::account_bank => true,

            // don't require auth
            EndPoint::items(_) => false,
            EndPoint::item_stats(_) => false,
            EndPoint::item_stats_all => false,
            EndPoint::recipes(_) => false,
            EndPoint::build => false,
        }
    }

    pub fn uri(&self) -> String {
        match self {
            EndPoint::account_materials => "account/materials".to_string(),
            EndPoint::account_bank => "account/bank".to_string(),
            EndPoint::items(id) => format!("items/{}", id.0.to_string()),
            _ => unreachable!(),
        }
    }
}

pub struct Requester {
    version: ApiVersion,
    pub base_uri: String,
    // todo: add in auth key thing
}

impl Requester {
    // todo: add in auth key as input parameter. 
    pub fn new(version: ApiVersion) -> Requester {
        let mut uri_str = String::new();
        uri_str += "https://api.guildwars2.com/v";
        uri_str += &version.0.to_string();
        return Requester {
            version: version,
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
    fn uri_building() {
        let r = Requester::new(ApiVersion(2));
        let result = r.build_uri(&EndPoint::account_bank);
        assert_eq!(result, "https://api.guildwars2.com/v2/account/bank");
    }
}
