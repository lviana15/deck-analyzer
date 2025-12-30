use std::env;

use dotenv::dotenv;

mod clash_client;
mod models;

use crate::clash_client::ClashClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_token = env::var("API_TOKEN").expect("Missing API_TOKEN");
    let base_url = env::var("BASE_URL").expect("Missing BASE_URL");

    // Get locations
    // Get clans by locationId
    // Get clan members by clanTag
    // Filter clan members by trophies
    let client = ClashClient::new(base_url, api_token).expect("Failed to create HTTP client");

    // Test ClashClient methods
    println!("Fetching locations...");
    let locations = client.get_locations().await?;
    println!("Locations: {:?}", locations);

    println!("Fetching clans by location...");
    if let Some(location) = locations.items.first() {
        let clans = client.get_clans_by_location(&location.id.to_string(), Some(10)).await?;
        println!("Clans: {:?}", clans);

        if let Some(clan) = clans.items.first() {
            println!("Fetching clan details...");
            let clan_details = client.get_clan(&clan.tag).await?;
            println!("Clan details: {:?}", clan_details);

            println!("Fetching player details...");
            if let Some(member_list) = &clan_details.member_list {
                if let Some(member) = member_list.first() {
                    let player = client.get_player(&member.tag).await?;
                    println!("Player details: {:?}", player);
                }
            }
        }
    }

    Ok(())
}
