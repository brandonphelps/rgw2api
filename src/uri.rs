#![allow(dead_code)]

// should be usize?

// todo: move this to some types thing?
struct ItemId(u128);
struct RecipeId(u128);

#[allow(non_camel_case_types)]
enum EndPoint {
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
    pub fn uri(self) -> String {
        match self {
            EndPoint::account_materials => "account/materials".to_string(),
            EndPoint::account_bank => "account/bank".to_string(),
            EndPoint::items(id) => format!("items/{}", id.0.to_string()),
            _ => unreachable!(),
        }
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
        assert_eq!(p.uri(), "items/1000");
    }
}
