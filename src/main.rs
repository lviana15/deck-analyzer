use dotenv::dotenv;
use futures::{StreamExt, stream};
use std::env;

mod clash_client;
mod models;

use crate::{clash_client::ClashClient, models::Clan};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_token = env::var("API_TOKEN").expect("Missing API_TOKEN");
    let base_url = env::var("BASE_URL").expect("Missing BASE_URL");
    let client = ClashClient::new(base_url, api_token).expect("Failed to create HTTP client");

    // Get locations
    // Get clans by locationId
    // Get clan members by clanTag
    // Filter clan members by trophies

    println!("Fetching locations...");
    let locations = client.get_locations().await?;

    let clans: Vec<Clan> = stream::iter(locations.iter())
        .map(async |loc| {
            client
                .get_clans_by_location(&loc.id.to_string(), Some(15))
                .await
                .ok()
        })
        .buffer_unordered(5)
        .filter_map(|res| async move { res })
        .flat_map(|page| stream::iter(page.items))
        .collect()
        .await;

    println!("{:#?}", clans);

    Ok(())
}
