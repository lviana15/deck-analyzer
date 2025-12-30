use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub is_country: bool,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub tag: String,
    pub name: String,
    #[serde(rename = "type")]
    pub clan_type: String,
    pub badge_id: i32,
    pub clan_score: i32,
    pub clan_war_trophies: i32,
    pub location: Location,
    pub required_trophies: i32,
    pub members: i32,
    pub member_list: Option<Vec<ClanMember>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub tag: String,
    pub name: String,
    pub trophies: i32,
    pub arena: Arena,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arena {
    pub id: i32,
    pub name: String,
    pub raw_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub tag: String,
    pub name: String,
    pub exp_level: i32,
    pub trophies: i32,
    pub best_trophies: i32,
    pub clan: Option<ClanInfo>,
    pub arena: Arena,
    pub current_deck: Vec<Card>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanInfo {
    pub tag: String,
    pub name: String,
    pub badge_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    name: String,
    id: i32,
    rarity: String,
}
