//! # Comment
//!
//! Comment request data

use serde_with::{serde_as, TimestampSeconds};
use std::time::SystemTime;

use crate::types::Comment;

#[derive(Debug, Deserialize)]
pub struct CommentResponse {
    pub data: CommentResponseData,
}

impl CommentResponse {
    pub fn end_cursor(&self) -> Option<&str> {
        self.data
            .shortcode_media
            .edge_media_to_comment
            .page_info
            .end_cursor
            .as_deref()
    }

    pub fn comments(self) -> Vec<Comment> {
        if let Some(edges) = self.data.shortcode_media.edge_media_to_comment.edges {
            edges.into_iter().map(|x| Comment::from(x.node)).collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CommentResponseData {
    pub shortcode_media: ShortcodeMedia,
}

#[derive(Debug, Deserialize)]
pub struct ShortcodeMedia {
    pub edge_media_to_comment: EdgeMediaToComment,
}

#[derive(Debug, Deserialize)]
pub struct EdgeMediaToComment {
    pub edges: Option<Vec<MediaToCommentEdge>>,
    pub page_info: EdgeMediaToCommentPageInfo,
}

#[derive(Debug, Deserialize)]
pub struct EdgeMediaToCommentPageInfo {
    pub end_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MediaToCommentEdge {
    pub node: MediaToCommentNode,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct MediaToCommentNode {
    pub id: String,
    pub text: String,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub created_at: SystemTime,
    pub owner: MediaToCommentOwner,
}

#[derive(Debug, Deserialize)]
pub struct MediaToCommentOwner {
    pub id: String,
    pub profile_pic_url: String,
    pub username: String,
}

impl From<MediaToCommentNode> for Comment {
    fn from(node: MediaToCommentNode) -> Self {
        Self {
            id: node.id,
            text: node.text,
            created_at: node.created_at,
            user_id: node.owner.id,
            username: node.owner.username,
            user_profile_pic: node.owner.profile_pic_url.replace("\\u0026", "&"),
        }
    }
}
