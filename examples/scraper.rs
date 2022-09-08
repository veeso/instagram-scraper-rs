use instagram_scraper_rs::InstagramScraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let username = std::env::var("INSTAGRAM_USERNAME").expect("missing env key INSTAGRAM_USERNAME");
    let password = std::env::var("INSTAGRAM_PASSWORD").expect("missing env key INSTAGRAM_PASSWORD");
    let mut scraper = InstagramScraper::default().authenticate_with_login(username, password);
    scraper.do_login().await?;
    scraper.logout().await?;
    Ok(())
}
