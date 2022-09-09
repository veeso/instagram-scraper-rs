use instagram_scraper_rs::InstagramScraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let profile = match std::env::args().nth(1) {
        Some(profile) => profile,
        None => {
            anyhow::bail!("usage: scraper <profile>")
        }
    };
    let username = std::env::var("INSTAGRAM_USERNAME").ok();
    let password = std::env::var("INSTAGRAM_PASSWORD").ok();
    let mut scraper = InstagramScraper::default();
    if let (Some(username), Some(password)) = (username, password) {
        println!("authenticating with username {}", username);
        scraper = scraper.authenticate_with_login(username, password);
    }
    scraper.login().await?;
    // get user info
    let user = scraper.scrape_userinfo(&profile).await?;
    println!("{}: {}", user.username, user.biography.unwrap_or_default());
    // logout
    let _ = scraper.logout().await;
    Ok(())
}
