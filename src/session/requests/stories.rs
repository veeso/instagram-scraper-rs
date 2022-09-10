//! # Stories
//!
//! request response types

use crate::{Story, StorySource};

use serde_with::{serde_as, TimestampSeconds};
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
pub struct ReelsMedia {
    pub data: ReelsMediaData,
}

impl ReelsMedia {
    /// Extract items from payload
    pub fn items(self) -> Vec<ReelsMediaItem> {
        let mut items = Vec::new();
        for media in self.data.reels_media {
            for item in media.items {
                items.push(item);
            }
        }
        items
    }
}

#[derive(Debug, Deserialize)]
pub struct ReelsMediaData {
    pub reels_media: Vec<ReelsMediaMedia>,
}

#[derive(Debug, Deserialize)]
pub struct ReelsMediaMedia {
    pub items: Vec<ReelsMediaItem>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ReelsMediaItem {
    pub display_resources: Vec<DisplayResources>,
    pub display_url: String,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub expiring_at_timestamp: SystemTime,
    pub id: String,
    pub is_video: bool,
    pub media_preview: Option<String>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub taken_at_timestamp: SystemTime,
}

#[derive(Debug, Deserialize)]
pub struct DisplayResources {
    pub config_height: usize,
    pub config_width: usize,
    pub src: String,
}

impl From<ReelsMediaItem> for Story {
    fn from(media: ReelsMediaItem) -> Self {
        Self {
            source_set: media
                .display_resources
                .into_iter()
                .map(|resource| StorySource {
                    height: resource.config_height,
                    url: resource.src.replace("\\u0026", "&"),
                    width: resource.config_width,
                })
                .collect(),
            url: media.display_url.replace("\\u0026", "&"),
            expiring_at_timestamp: media.expiring_at_timestamp,
            id: media.id,
            is_video: media.is_video,
            media_preview: media.media_preview,
            taken_at_timestamp: media.taken_at_timestamp,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HighlightReels {
    pub data: HighlightReelsData,
}

impl HighlightReels {
    /// Collect ids from nodes
    pub fn node_ids(&self) -> Vec<String> {
        self.data
            .user
            .edge_highlight_reels
            .edges
            .iter()
            .map(|e| e.node.id.clone())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct HighlightReelsData {
    user: HighlightReelsUser,
}

#[derive(Debug, Deserialize)]
pub struct HighlightReelsUser {
    pub edge_highlight_reels: HighlightReelsEdges,
}

#[derive(Debug, Deserialize)]
pub struct HighlightReelsEdges {
    pub edges: Vec<HighlightReelsEdge>,
}

#[derive(Debug, Deserialize)]
pub struct HighlightReelsEdge {
    pub node: HighlightReelsEdgeNode,
}

#[derive(Debug, Deserialize)]
pub struct HighlightReelsEdgeNode {
    pub id: String,
}
