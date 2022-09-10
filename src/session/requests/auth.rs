//! # Login
//!
//! Login related requests

use std::time::{SystemTime, UNIX_EPOCH};

/// Request body for username password login
#[derive(Debug)]
pub struct UsernamePasswordLoginRequest {
    username: String,
    enc_password: String,
    query_params: String,
    opt_into_one_tap: String,
}

impl UsernamePasswordLoginRequest {
    pub fn new(username: String, password: String) -> Self {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let enc_password = format!("#PWD_INSTAGRAM_BROWSER:0:{}:{}", timestamp, password);
        Self {
            username,
            enc_password,
            query_params: "{}".to_string(),
            opt_into_one_tap: "false".to_string(),
        }
    }

    pub fn form(self) -> Vec<(String, String)> {
        vec![
            ("username".to_string(), self.username),
            ("enc_password".to_string(), self.enc_password),
            ("queryParams".to_string(), self.query_params),
            ("optIntoOneTap".to_string(), self.opt_into_one_tap),
        ]
    }
}

/// Response for username password login
#[derive(Deserialize, Debug)]
pub struct UsernamePasswordLoginResponse {
    pub authenticated: bool,
    pub status: Option<String>,
    pub message: Option<String>,
}

/// Request to logout
#[derive(Serialize, Debug)]
pub struct LogoutRequest {
    csrfmiddlewaretoken: String,
}

impl LogoutRequest {
    pub fn new(csrfmiddlewaretoken: String) -> Self {
        Self {
            csrfmiddlewaretoken,
        }
    }

    pub fn form(self) -> Vec<(String, String)> {
        vec![("csrfmiddlewaretoken".to_string(), self.csrfmiddlewaretoken)]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_build_username_password_login_request() {
        let req = UsernamePasswordLoginRequest::new("pippo".to_string(), "secret".to_string());
        assert_eq!(&req.username, "pippo");
        let enc_password = req.enc_password.clone();
        assert_eq!(
            req.form(),
            vec![
                ("username".to_string(), "pippo".to_string()),
                ("enc_password".to_string(), enc_password),
                ("queryParams".to_string(), "{}".to_string()),
                ("optIntoOneTap".to_string(), "false".to_string()),
            ]
        );
    }

    #[test]
    fn should_build_logout_request() {
        let req = LogoutRequest::new("token".to_string());
        assert_eq!(&req.csrfmiddlewaretoken, "token");
        assert_eq!(
            req.form(),
            vec![("csrfmiddlewaretoken".to_string(), "token".to_string()),]
        );
    }
}
