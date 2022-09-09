//! # User
//!
//! Query data related to user fetch

use crate::User;

/// Describes the web profile query response
#[derive(Debug, Deserialize)]
pub struct WebProfileResponse {
    pub data: WebProfileData,
}

#[derive(Debug, Deserialize)]
pub struct WebProfileData {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub user: UserInfoUser,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoUser {
    pub has_anonymous_profile_picture: Option<bool>,
    pub hd_profile_pic_url_info: Image,
    pub hd_profile_pic_versions: Option<Vec<Image>>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub url: Option<String>,
}
