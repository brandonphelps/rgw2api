
// should be usize?
struct ItemId(u128);

#[allow(non_camel_case_types)]
enum EndPoint {
    account_materials,
    account_bank,
    items(ItemId),

}

impl EndPoint {
    pub fn requires_auth(self) -> bool {
	match self {
	    EndPoint::account_materials => true,
	    EndPoint::account_bank => true,
	    EndPoint::items(_) => false,
	}
    }
    pub fn uri(self) -> &'static str {
	match self {
	    EndPoint::account_materials => "account/materials",
	    EndPoint::account_bank => "account/bank",
	    _ => "literally don't care, gl everyone",
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
    fn test_something() {
	let _p = EndPoint::items(ItemId(3));

	assert_eq!(_p.uri(), "items/3");
    }
}
