//! # Requests
//!
//! This module exposes the different types to send requests

use const_format::concatcp;

mod auth;
pub use auth::{LogoutRequest, UsernamePasswordLoginRequest, UsernamePasswordLoginResponse};

// -- constrants
pub const X_CSRF_TOKEN: &str = "X-CSRFToken";
pub const BASE_URL: &str = "https://www.instagram.com/";
pub const LOGIN_URL: &str = concatcp!(BASE_URL, "accounts/login/ajax/");
pub const LOGOUT_URL: &str = concatcp!(BASE_URL, "accounts/logout/");
pub const CHROME_WIN_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36";

pub const STORIES_USER_AGENT: &str = "Instagram 123.0.0.21.114 (iPhone; CPU iPhone OS 11_4 like Mac OS X; en_US; en-US; scale=2.00; 750x1334) AppleWebKit/605.1.15";
