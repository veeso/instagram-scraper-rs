#![crate_name = "instagram_scraper_rs"]
#![crate_type = "lib"]

//! # Instagram-scraper-rs
//!
//! TODO:
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod errors;
mod session;
mod types;

use session::Session;
use types::Authentication;

// exports
pub use errors::{InstagramScraperError, InstagramScraperResult};
pub use types::{Post, Stories, Story, StorySource, User};

/// instagram scraper client
pub struct InstagramScraper {
    auth: Authentication,
    session: Session,
}

impl InstagramScraper {
    /// Configure scraper to authenticate with username/password
    pub fn authenticate_with_login(
        mut self,
        username: impl ToString,
        password: impl ToString,
    ) -> Self {
        self.auth = Authentication::UsernamePassword {
            username: username.to_string(),
            password: password.to_string(),
        };
        self
    }

    /// Login to instagram
    pub async fn login(&mut self) -> InstagramScraperResult<()> {
        self.session.login(self.auth.clone()).await
    }

    /// Logout from instagram account
    pub async fn logout(&mut self) -> InstagramScraperResult<()> {
        debug!("signin out from Instagram");
        self.session.logout().await?;
        debug!("logout ok, reinitializing session");
        // re-initialize session
        self.session = Session::default();
        Ok(())
    }

    /// Scrape profile HD picture if any. Returns the URL.
    /// The user id can be retrieved with `scrape_userinfo`
    pub async fn scrape_profile_pic(
        &mut self,
        user_id: &str,
    ) -> InstagramScraperResult<Option<String>> {
        self.session.scrape_profile_pic(user_id).await
    }

    /// Scrape profile HD picture if any. Returns the URL.
    /// The user id can be retrieved with `scrape_userinfo`
    /// You can provide the maximum amount of posts to fetch. Use usize::MAX to get all the available stproes.
    /// Keep in mind that a GET request will be sent each 3 highlighted stories.
    pub async fn scrape_user_stories(
        &mut self,
        user_id: &str,
        max_highlight_stories: usize,
    ) -> InstagramScraperResult<Stories> {
        self.session
            .scrape_stories(user_id, max_highlight_stories)
            .await
    }

    /// Scrape user info
    pub async fn scrape_userinfo(&mut self, username: &str) -> InstagramScraperResult<User> {
        self.session.scrape_shared_data_userinfo(username).await
    }

    /// Scrape posts from user.
    /// You can provide the maximum amount of posts to fetch. Use usize::MAX to get all the available posts.
    /// Keep in mind that a GET request will be sent each 50 posts.
    pub async fn scrape_posts(
        &mut self,
        user_id: &str,
        max_posts: usize,
    ) -> InstagramScraperResult<Vec<Post>> {
        if max_posts == 0 {
            warn!("max_posts is 0; return empty vector");
            return Ok(vec![]);
        }
        self.session.scrape_posts(user_id, max_posts).await
    }
}

impl Default for InstagramScraper {
    fn default() -> Self {
        Self {
            auth: Authentication::Guest,
            session: Session::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn should_login_and_logout() {
        let mut scraper = InstagramScraper::default();
        assert!(scraper.login().await.is_ok());
        assert!(scraper.logout().await.is_ok());
    }

    #[tokio::test]
    async fn should_return_empty_vec_if_scraping_0_posts() {
        let mut scraper = InstagramScraper::default();
        assert!(scraper.scrape_posts("aaa", 0).await.unwrap().is_empty());
    }
}
