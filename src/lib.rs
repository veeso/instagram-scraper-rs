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

pub use errors::{InstagramScraperError, InstagramScraperResult};
use session::Session;

#[derive(Debug, Clone)]
/// Defines the user authentication method
enum Authentication {
    UsernamePassword { username: String, password: String },
    Guest,
}

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

    pub async fn do_login(&mut self) -> InstagramScraperResult<()> {
        self.login().await
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

    /// Login to instagram
    async fn login(&mut self) -> InstagramScraperResult<()> {
        self.session.login(self.auth.clone()).await
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
