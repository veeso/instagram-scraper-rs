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
    println!(
        "{}: {} (followers: {}; following {}) - user id: {}",
        user.username,
        user.biography.as_deref().unwrap_or_default(),
        user.followers(),
        user.following(),
        user.id
    );
    // get user stories
    let stories = scraper.scrape_user_stories(&user.id, 10).await?;
    println!(
        "there are {} stories for {}",
        stories.main_stories.len(),
        profile
    );
    println!(
        "there are {} highlighted stories for {}",
        stories.highlight_stories.len(),
        profile
    );
    // get posts
    let posts = scraper.scrape_posts(&user.id, 10).await?;
    println!("there are {} posts for {}", posts.len(), profile);
    if let Some(post) = posts.get(0) {
        println!(
            "latest post: {}",
            post.caption.as_deref().unwrap_or_default()
        );
        // get comments
        let comments = scraper.scrape_comments(post, 5).await?;
        println!("latest 5 comments to the post: ");
        for comment in comments {
            println!("{}: {}", comment.username, comment.text);
        }
    }

    // logout
    scraper
        .logout()
        .await
        .map_err(|e| anyhow::anyhow!("logout failed: {}", e))
}
