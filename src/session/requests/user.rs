//! # User
//!
//! Query data related to user fetch

/// Describes the web profile query response
#[derive(Debug, Deserialize)]
pub struct WebProfileResponse {
    pub data: WebProfileData,
}

#[derive(Debug, Deserialize)]
pub struct WebProfileData {
    pub user: User,
}

/// Describes the web profile query response
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub biography: Option<String>,
    pub blocked_by_viewer: bool,
    pub business_category_name: Option<String>,
    pub business_email: Option<String>,
    pub business_phone_number: Option<String>,
    pub category_name: Option<String>,
    pub country_block: bool,
    pub external_url_linkshimmed: Option<String>,
    pub external_url: Option<String>,
    pub fbid: Option<String>,
    pub followed_by_viewer: bool,
    pub follows_viewer: bool,
    pub full_name: String,
    pub has_ar_effects: bool,
    pub has_blocked_viewer: bool,
    pub has_channel: bool,
    pub has_clips: bool,
    pub has_guides: bool,
    pub has_requested_viewer: bool,
    pub hide_like_and_view_counts: bool,
    pub highlight_reel_count: isize,
    pub id: String,
    pub is_business_account: bool,
    pub is_eligible_to_view_account_transparency: bool,
    pub is_embeds_disabled: bool,
    pub is_guardian_of_viewer: bool,
    pub is_joined_recently: bool,
    pub is_private: bool,
    pub is_professional_account: bool,
    pub is_supervised_by_viewer: bool,
    pub is_supervised_user: bool,
    pub is_supervision_enabled: bool,
    pub is_verified: bool,
    pub overall_category_name: Option<String>,
    pub profile_pic_url_hd: Option<String>,
    pub profile_pic_url: Option<String>,
    pub requested_by_viewer: bool,
    pub should_show_category: bool,
    pub should_show_public_contacts: bool,
    pub username: String,
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
