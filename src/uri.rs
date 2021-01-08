use std::ops;


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
	    EndPoint::account_materials => true,
	    EndPoint::account_bank => true,
	    EndPoint::items(_) => false,
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


fn account_materials() {
    let uri = "account/materials";
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
