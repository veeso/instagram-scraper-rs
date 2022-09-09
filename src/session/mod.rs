//! # Session
//!
//! This module exposes the session for the instagram client

use crate::{Authentication, InstagramScraperError, InstagramScraperResult};

use reqwest::{header, Client, ClientBuilder};

mod requests;
use requests::{BASE_URL, LOGIN_URL, LOGOUT_URL, STORIES_USER_AGENT, X_CSRF_TOKEN};

pub use requests::User;

/// The session is a storage for values required by the instagram client to work.
/// It also exposes the instagram HTTP client
#[derive(Debug)]
pub struct Session {
    csrftoken: Option<String>,
    client: Client,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            csrftoken: None,
            client: ClientBuilder::new()
                .cookie_store(true)
                .user_agent(STORIES_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

impl Session {
    /// Login into instagram account or as a guest based on provided authentication type
    pub(crate) async fn login(
        &mut self,
        authentication: Authentication,
    ) -> InstagramScraperResult<()> {
        let token = match authentication {
            Authentication::Guest => self.login_as_guest().await?,
            Authentication::UsernamePassword { username, password } => {
                self.login_as_user(username, password).await?
            }
        };
        debug!("login successful; csrf token: {}", token);
        self.csrftoken = Some(token);
        Ok(())
    }

    /// Scrape profile picture for provided username.
    ///
    /// Returns the image url
    pub async fn scrape_profile_pic(
        &mut self,
        user_id: &str,
    ) -> InstagramScraperResult<Option<String>> {
        self.restrict_authed()?;
        let response = self
            .client
            .get(format!(
                "https://i.instagram.com/api/v1/users/{}/info/",
                user_id.to_string()
            ))
            .send()
            .await?;
        Self::restrict_successful(&response)?;
        let user_info = response
            .text()
            .await
            .map(|t| serde_json::from_str::<requests::UserInfoResponse>(&t).map(|u| u.user))?;
        let user_info = user_info?;
        if user_info.has_anonymous_profile_picture.unwrap_or_default() {
            debug!("user has anonymous profile picture");
            return Ok(None);
        }
        if let Some(url) = user_info.hd_profile_pic_url_info.url {
            debug!("found hd profile pic {}", url);
            Ok(Some(url))
        } else {
            debug!("searching user profile pic in versions");
            Ok(user_info
                .hd_profile_pic_versions
                .map(|images| {
                    images
                        .into_iter()
                        .rev()
                        .find(|img| img.url.is_some())
                        .map(|img| img.url.unwrap())
                })
                .flatten())
        }
    }

    /// Scrape shared data for user
    pub async fn scrape_shared_data_userinfo(
        &mut self,
        username: &str,
    ) -> InstagramScraperResult<User> {
        self.restrict_authed()?;
        debug!("collecting user info for {}", username);
        let response = self
            .client
            .get(format!(
                "https://i.instagram.com/api/v1/users/web_profile_info/?username={}",
                username
            ))
            .send()
            .await?;
        Self::restrict_successful(&response)?;
        match response
            .text()
            .await
            .map(|t| serde_json::from_str::<requests::WebProfileResponse>(&t).map(|i| i.data.user))
        {
            Err(err) => Err(err.into()),
            Ok(Ok(user)) => Ok(user),
            Ok(Err(err)) => Err(err.into()),
        }
    }

    // -- private

    /// Logout from Instagram
    pub(crate) async fn logout(&mut self) -> InstagramScraperResult<()> {
        if let Some(csrf_token) = self.csrftoken.as_deref() {
            let response = self
                .client
                .post(LOGOUT_URL)
                .form(&requests::LogoutRequest::new(csrf_token.to_string()).to_form())
                .header(header::REFERER, BASE_URL)
                .header(X_CSRF_TOKEN, csrf_token.to_string())
                .header("X-Requested-With", "XMLHttpRequest")
                .send()
                .await?;
            Self::restrict_successful(&response)
        } else {
            error!("unauthenticated user; cannot logout");
            Err(InstagramScraperError::Unauthenticated)
        }
    }

    /// Returns whether session is authed
    pub(crate) fn authed(&self) -> bool {
        self.csrftoken.is_some()
    }

    /// Login to instagram as a guest
    async fn login_as_guest(&self) -> InstagramScraperResult<String> {
        debug!("authenticating as guest");
        self.request_csrftoken().await
    }

    /// Login to instagram as an authenticated user
    async fn login_as_user(
        &mut self,
        username: String,
        password: String,
    ) -> InstagramScraperResult<String> {
        debug!("authenticating with username and password");
        let token = self.request_csrftoken().await?;
        let response = self
            .client
            .post(LOGIN_URL)
            .form(
                requests::UsernamePasswordLoginRequest::new(username, password)
                    .to_form()
                    .as_slice(),
            )
            .header(header::REFERER, BASE_URL)
            .header(header::USER_AGENT, STORIES_USER_AGENT)
            .header(X_CSRF_TOKEN, token.clone())
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?;
        Self::restrict_successful(&response)?;
        debug!("setting cookies received from response");
        let body: requests::UsernamePasswordLoginResponse = response.json().await?;
        if body.authenticated {
            debug!("user authenticated successfully");
            Ok(token)
        } else {
            error!("login failed: {:?}; {:?}", body.status, body.message);
            Err(InstagramScraperError::AuthenticationFailed {
                status: body.status.unwrap_or_default(),
                message: body.message.unwrap_or_default(),
            })
        }
    }

    async fn request_csrftoken(&self) -> InstagramScraperResult<String> {
        let response = self
            .client
            .get(BASE_URL)
            .header(header::REFERER, BASE_URL)
            .header(header::USER_AGENT, STORIES_USER_AGENT)
            .send()
            .await?;
        Self::restrict_successful(&response)?;
        trace!("login status: {}", response.status());
        let mut cookies = response.cookies();
        match cookies
            .find(|x| x.name() == "csrftoken")
            .map(|x| x.value().to_string())
        {
            Some(cookie) => Ok(cookie),
            None => Err(InstagramScraperError::CsrfTokenIsMissing),
        }
    }

    /// This function puts a restriction on a function flow to return in case of an unsuccessful status code in the HTTP response.
    ///
    /// it must be called as `Self::restrict_successful(&response)?;`
    fn restrict_successful(response: &reqwest::Response) -> InstagramScraperResult<()> {
        debug!("response status {}", response.status());
        match response.status().is_success() {
            true => Ok(()),
            false => Err(InstagramScraperError::from(response.status())),
        }
    }

    /// This function puts a restriction on a function flow to return in case we're not authenticated
    fn restrict_authed(&self) -> InstagramScraperResult<()> {
        trace!("checking authentication");
        if self.authed() {
            trace!("authed");
            Ok(())
        } else {
            error!("unauthenticated user, but authentication is required");
            Err(InstagramScraperError::Unauthenticated)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_initialize_session() {
        let session = Session::default();
        assert!(session.csrftoken.is_none());
        assert!(!session.authed());
    }

    #[tokio::test]
    async fn should_login_as_guest() {
        let mut session = Session::default();
        assert!(session.login(Authentication::Guest).await.is_ok());
        assert!(session.authed());
    }

    #[cfg(not(feature = "github-ci"))]
    #[tokio::test]
    async fn should_login_as_user() {
        let username =
            std::env::var("INSTAGRAM_USERNAME").expect("missing env key INSTAGRAM_USERNAME");
        let password =
            std::env::var("INSTAGRAM_PASSWORD").expect("missing env key INSTAGRAM_PASSWORD");
        let mut session = Session::default();
        assert!(session
            .login(Authentication::UsernamePassword { username, password })
            .await
            .is_ok());
        assert!(session.authed());
    }

    #[tokio::test]
    async fn should_scrape_user_profile_picture() {
        let mut session = Session::default();
        assert!(session.login(Authentication::Guest).await.is_ok());
        let user_id = session
            .scrape_shared_data_userinfo("bigluca.marketing")
            .await
            .unwrap()
            .id;
        assert!(session
            .scrape_profile_pic(&user_id)
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn should_scrape_shared_userinfo() {
        let mut session = Session::default();
        assert!(session.login(Authentication::Guest).await.is_ok());
        assert_eq!(
            session
                .scrape_shared_data_userinfo("bigluca.marketing")
                .await
                .unwrap()
                .username
                .as_str(),
            "bigluca.marketing"
        );
    }
}
