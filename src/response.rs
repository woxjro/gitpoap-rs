use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PoapResponse {
    #[serde(rename = "isGitPOAP")]
    pub is_gitpoap: bool,
    #[serde(rename = "gitPOAPId")]
    pub gitpoap_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct PoapIdsResponse {
    #[serde(rename = "poapTokenIds")]
    pub poap_token_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EventResponse {
    #[serde(rename = "isGitPOAP")]
    pub is_gitpoap: bool,
    #[serde(rename = "gitPOAPId")]
    pub gitpoap_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct PoapEventFancyIdsResponse {
    #[serde(rename = "poapEventFancyIds")]
    pub poap_event_fancy_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GitPoapEvent {
    #[serde(rename = "gitPoapEventId")]
    pub git_poap_event_id: i32,
    #[serde(rename = "poapEventId")]
    pub poap_event_id: i32,
    #[serde(rename = "poapEventFancyId")]
    pub poap_event_fancy_id: String,
    pub name: String,
    pub year: i32,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub repositories: Vec<String>,
    #[serde(rename = "mintedCount")]
    pub minted_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct AddressesResponse {
    pub addresses: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct GitpoapResponse {
    #[serde(rename = "gitPoapId")]
    pub git_poap_id: i32,
    #[serde(rename = "gitPoapEventId")]
    pub git_poap_event_id: i32,
    #[serde(rename = "poapTokenId")]
    pub poap_token_id: String,
    #[serde(rename = "poapEventId")]
    pub poap_event_id: i32,
    #[serde(rename = "poapEventFancyId")]
    pub poap_event_fancy_id: String,
    pub name: String,
    pub year: i32,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub repositories: Vec<String>,
    #[serde(rename = "earnedAt")]
    pub earned_at: String,
    #[serde(rename = "mintedAt")]
    pub minted_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GitpoapsResponse(pub Vec<GitpoapResponse>);
