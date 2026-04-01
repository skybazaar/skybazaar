use skybazaar::hypixel::client::HypixelClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HypixelClient::new(None, "bulibazaar_client")?;
    let page0 = client.auctions.fetch_page(0).await?;

    println!("page={} / total_pages={}", page0.page, page0.total_pages);
    println!("auctions_on_page={}", page0.auctions.len());

    for auction in page0.auctions.iter() {
        println!(
            "{} | price={:.0} | bin={}",
            auction.item_name.as_deref().unwrap_or("<missing item_name>"),
            auction.starting_bid.unwrap_or(0.0),
            auction.bin.unwrap_or(false)
        );
    }

    println!("items_fetched: {}", page0.auctions.len());

    Ok(())
}
