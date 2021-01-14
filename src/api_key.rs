extern crate bitflags;
extern crate serde_derive;

use serde_derive::Deserialize;

/// Contains an API key and information about the key and its user.
#[derive(Debug, Clone)]
pub struct APIKey {
    /// The API key.
    key: String,
    /// The permissions of the API key.
    permissions: Permission,
    /// The name that the user gave the API key when they created it.
    key_name: String,
    /// A non-changing unique id for the account that this API key is for.
    account_id: String,
    /// The account's name.  This can (very rarely) change.
    account_name: String,
    /// What content the account can access.
    access: std::collections::HashSet<String>,
}

bitflags::bitflags! {
    /// The possible permissions that an API key can have.
    pub struct Permission: u32 {
        const ACCOUNT =     (1 << 0);
        const BUILDS =      (1 << 1);
        const CHARACTERS =  (1 << 2);
        const GUILDS =      (1 << 3);
        const INVENTORIES = (1 << 4);
        const PROGRESSION = (1 << 5);
        const PVP =         (1 << 6);
        const TRADINGPOST = (1 << 7);
        const UNLOCKS =     (1 << 8);
        const WALLET =      (1 << 9);
    }
}

impl Permission {
    fn from_string(string: &str) -> Self {
        match string {
            "account" => Self::ACCOUNT,
            "builds" => Self::BUILDS,
            "characters" => Self::CHARACTERS,
            "guilds" => Self::GUILDS,
            "inventories" => Self::INVENTORIES,
            "progression" => Self::PROGRESSION,
            "pvp" => Self::PVP,
            "tradingpost" => Self::TRADINGPOST,
            "unlocks" => Self::UNLOCKS,
            "wallet" => Self::WALLET,
            _ => Self::empty(),
        }
    }

    fn from_strings(strings: Vec<String>) -> Self {
        let mut permissions = Self::empty();
        for string in strings {
            permissions |= Self::from_string(string.as_str());
        }
        permissions
    }
}

/// Payload from /v2/tokeninfo endpoint.
#[derive(Deserialize)]
struct TokenInfo {
    /// The name that the user gave the API key when they created it.
    name: String,
    /// The permissions the user gave this API key.
    permissions: Vec<String>,
}

/// Payload from /v2/account endpoint.
#[derive(Deserialize)]
struct AccountInfo {
    /// Unique, never changing id.
    id: String,
    /// Account name, can very rarely change.
    name: String,
    /// The user's access to content.
    /// What expansions they own and if they are free to play.
    access: Vec<String>,
}

impl APIKey {
    /// Creates a new API key.
    pub fn new(key: &str) -> reqwest::Result<Self> {
        let token_info: TokenInfo = reqwest::blocking::Client::new()
            .get("https://api.guildwars2.com/v2/tokeninfo")
            .header("Authorization", "Bearer ".to_owned() + key)
            .send()?
            .json()?;
        let account_info: AccountInfo = reqwest::blocking::Client::new()
            .get("https://api.guildwars2.com/v2/account")
            .header("Authorization", "Bearer ".to_owned() + key)
            .send()?
            .json()?;
        Ok(Self::from_data(key, token_info, account_info))
    }

    /// Creates a new API key from the data fetched from the endpoint.
    fn from_data(key: &str, token_info: TokenInfo, account_info: AccountInfo) -> Self {
        Self {
            key: key.to_owned(),
            permissions: Permission::from_strings(token_info.permissions),
            key_name: token_info.name,
            account_id: account_info.id,
            account_name: account_info.name,
            access: account_info.access.into_iter().collect(),
        }
    }

    /// Gets the api key as a string.
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// Gets the permissions of the API key.
    pub fn permissions(&self) -> Permission {
        self.permissions
    }

    /// Gets the name that the user gave the API key when they created it.
    pub fn key_name(&self) -> String {
        self.key_name.clone()
    }

    /// Gets the non-changing unique id for the account that this API key is for.
    pub fn account_id(&self) -> String {
        self.account_id.clone()
    }

    /// Gets the account's name.  This can (very rarely) change.
    pub fn account_name(&self) -> String {
        self.account_name.clone()
    }

    /// Gets what content the account for this API key has access to.
    pub fn access(&self) -> std::collections::HashSet<String> {
        self.access.clone()
    }
}

impl std::fmt::Display for APIKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// Basic test for the internal from_data constructor and all getters.
    #[test]
    fn from_data() {
        let key_string = "the_key";
        let token_info = TokenInfo {
            name: "api_key_name".into(),
            permissions: vec!["account".into(), "pvp".into()],
        };
        let account_info = AccountInfo {
            id: "account_id".into(),
            name: "account_name".into(),
            access: vec!["PlayForFree".into(), "GuildWars2".into()],
        };
        let key = APIKey::from_data(key_string, token_info, account_info);

        assert_eq!("the_key", key.key());
        assert_eq!("the_key", key.to_string());
        assert_eq!(Permission::ACCOUNT | Permission::PVP, key.permissions());
        assert_eq!("api_key_name", key.key_name());
        assert_eq!("account_id", key.account_id());
        assert_eq!("account_name", key.account_name());
        assert_eq!(2, key.access().len());
        assert!(key.access().contains("PlayForFree"));
        assert!(key.access().contains("GuildWars2"));
    }

    /// Unknown permissions get ignored.
    #[test]
    fn unknown_permission() {
        assert_eq!(Permission::empty(), Permission::from_string("123456"));
    }
}
