//! # Types
//!
//! Defines the return types for the scraper

use std::time::SystemTime;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Post {}

/// Instagram stories
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Stories {
    /// Main stories
    pub main_stories: Vec<Story>,
    /// Highlight stories are the permament stories stored on the profile
    pub highlight_stories: Vec<Story>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Story {
    pub source_set: Vec<StorySource>,
    pub url: String,
    pub expiring_at_timestamp: SystemTime,
    pub id: String,
    pub is_video: bool,
    pub media_preview: Option<String>,
    pub taken_at_timestamp: SystemTime,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct StorySource {
    pub height: usize,
    pub url: String,
    pub width: usize,
}

/// Describes the web profile query response
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct User {
    pub biography: Option<String>,
    pub blocked_by_viewer: bool,
    pub business_category_name: Option<String>,
    pub business_email: Option<String>,
    pub business_phone_number: Option<String>,
    pub category_name: Option<String>,
    pub country_block: bool,
    edge_followed_by: FollowData,
    edge_follow: FollowData,
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

impl User {
    /// Get followers count
    pub fn followers(&self) -> usize {
        self.edge_followed_by.count
    }

    /// Get following count
    pub fn following(&self) -> usize {
        self.edge_follow.count
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FollowData {
    count: usize,
}

#[derive(Debug, Clone)]
/// Defines the user authentication method
pub enum Authentication {
    UsernamePassword { username: String, password: String },
    Guest,
}
