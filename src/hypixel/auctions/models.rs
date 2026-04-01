use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuctionApiModel {
    pub uuid: String,
    #[serde(default, alias = "itemName")]
    pub item_name: String,
    #[serde(default, alias = "startingBid")]
    pub starting_bid: f64,
    #[serde(default)]
    pub bin: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionsPageResponse {
    pub success: bool,
    pub page: u32,
    pub total_pages: u32,
    pub total_auctions: u32,
    pub last_updated: u64,
    pub auctions: Vec<AuctionApiModel>,
}
