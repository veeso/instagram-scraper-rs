//! # Post
//!
//! Post requests types

use crate::Post;

use serde_with::{serde_as, TimestampSeconds};
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
pub struct PostResponse {
    pub data: PostResponseData,
}

impl PostResponse {
    pub fn end_cursor(&self) -> Option<&str> {
        self.data
            .user
            .edge_owner_to_timeline_media
            .page_info
            .end_cursor
            .as_deref()
    }

    pub fn posts(self) -> Vec<Post> {
        self.data
            .user
            .edge_owner_to_timeline_media
            .edges
            .into_iter()
            .map(|edge| Post::from(edge.node))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct PostResponseData {
    pub user: PostResponseUser,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseUser {
    pub edge_owner_to_timeline_media: PostResponseTimelineMedia,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseTimelineMedia {
    pub edges: Vec<PostResponseEdge>,
    pub page_info: PostResponsePageInfo,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseEdge {
    pub node: PostResponseNode,
}

#[derive(Debug, Deserialize)]
pub struct PostResponsePageInfo {
    pub end_cursor: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct PostResponseNode {
    pub id: String,
    pub edge_media_to_caption: PostResponseCaption,
    pub comments_disabled: bool,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub taken_at_timestamp: SystemTime,
    pub dimensions: PostResponseDimensions,
    pub display_url: String,
    /// Comments amount
    pub edge_media_to_comment: PostResponseNodeCounter,
    /// Likes amount
    pub edge_media_preview_like: PostResponseNodeCounter,
    pub media_preview: Option<String>,
    pub shortcode: String,
    pub thumbnail_src: String,
    pub is_video: bool,
    pub video_view_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseDimensions {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseNodeCounter {
    pub count: isize,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseCaption {
    pub edges: Vec<PostResponseCaptionEdge>,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseCaptionEdge {
    pub node: PostResponseCaptionNode,
}

#[derive(Debug, Deserialize)]
pub struct PostResponseCaptionNode {
    pub text: String,
}

impl From<PostResponseNode> for Post {
    fn from(node: PostResponseNode) -> Self {
        let caption = node
            .edge_media_to_caption
            .edges
            .into_iter()
            .map(|x| x.node.text)
            .next();
        Self {
            caption,
            comments_disabled: node.comments_disabled,
            comments: if node.edge_media_to_comment.count < 0 {
                None
            } else {
                Some(node.edge_media_to_comment.count as usize)
            },
            display_url: node.display_url.replace("\\u0026", "&"),
            height: node.dimensions.height,
            id: node.id,
            is_video: node.is_video,
            likes: if node.edge_media_preview_like.count < 0 {
                None
            } else {
                Some(node.edge_media_preview_like.count as usize)
            },
            media_preview: node.media_preview,
            shortcode: node.shortcode,
            taken_at_timestamp: node.taken_at_timestamp,
            thumbnail_src: node.thumbnail_src.replace("\\u0026", "&"),
            video_view_count: node.video_view_count.unwrap_or_default(),
            width: node.dimensions.width,
        }
    }
}
