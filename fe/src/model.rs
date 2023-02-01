use crate::util::fetch_github;
use wasm_bindgen::prelude::*;
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
    name: Option<String>,
    company: Option<String>,
    blog: String,
    location: Option<String>,
    email: Option<String>,
    hireable: Option<bool>,
    bio: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    created_at: String,
    updated_at: String,
}

impl User {
    pub async fn fetch(user: &str) -> Result<Self, JsValue> {
        let user = fetch_github(&format!("https://api.github.com/users/{}", user)).await?;
        Ok(serde_json::from_str::<Self>(&user).unwrap())
    }
}
