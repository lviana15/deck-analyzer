use crate::models::{Player, types::{Clan, ClanMember, Location}};
use reqwest::{Client, header};
use serde::Deserialize;
use std::error::Error;
use urlencoding::encode;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    pub cursors: Option<Cursors>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursors {
    pub before: Option<String>,
    pub after: Option<String>,
}

pub struct ClashClient {
    pub http: Client,
    pub base_url: String,
}

impl ClashClient {
    pub fn new(base_url: String, api_token: String) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", api_token))
            .expect("Failed to set auth header");

        auth_value.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            http: client,
            base_url,
        })
    }

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    async fn send_request<T: for<'de> Deserialize<'de>>(&self, url: &str, query: Option<&[(&str, &str)]>) -> Result<T, Box<dyn Error>> {
        let request = match query {
            Some(params) => self.http.get(url).query(params),
            None => self.http.get(url),
        };

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()).into());
        }

        let json_data = response.json::<T>().await?;
        Ok(json_data)
    }

    pub async fn get_locations(&self) -> Result<PaginatedResponse<Location>, Box<dyn Error>> {
        let url = self.endpoint("locations");
        self.send_request(&url, None).await
    }

    pub async fn get_clans_by_location(
        &self,
        location_id: &str,
        min_members: Option<u32>,
    ) -> Result<PaginatedResponse<Clan>, Box<dyn Error>> {
        let url = self.endpoint("clans");
        let min_value = min_members.unwrap_or(0).to_string();

        let query_params = if min_value == "0" {
            vec![("locationId", location_id)]
        } else {
            vec![("locationId", location_id), ("minMembers", &min_value)]
        };

        self.send_request(&url, Some(&query_params)).await
    }

    pub async fn get_clan(&self, clan_tag: &str) -> Result<Clan, Box<dyn Error>> {
        let url = self.endpoint(&format!("clans/{}", encode(clan_tag)));
        self.send_request(&url, None).await
    }

    pub async fn get_player(&self, player_tag: &str) -> Result<Player, Box<dyn Error>> {
        let url = self.endpoint(&format!("players/{}", encode(player_tag)));
        self.send_request(&url, None).await
    }
}
