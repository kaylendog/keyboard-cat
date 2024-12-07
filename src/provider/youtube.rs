use std::time::Duration;

use tracing::warn;

use super::{Metadata, Provider};

/// An audio provider for YouTube.
pub struct YoutubeProvider;

impl Provider for YoutubeProvider {
    type Error = YoutubeProviderError;

    fn matches(&self, url: &str) -> bool {
        url.starts_with("https://www.youtube.com/watch?v=")
    }

    async fn source(&self, metadata: Metadata) -> Result<songbird::input::Input, Self::Error> {
        todo!()
    }
}

pub struct YoutubeSource {
    inner: rusty_ytdl::Video,
    info: rusty_ytdl::VideoInfo,
}

impl From<rusty_ytdl::VideoDetails> for Metadata {
    fn from(info: rusty_ytdl::VideoDetails) -> Self {
        Self {
            title: info.title,
            description: info.description,
            image_url: info.thumbnails.first().map(|t| t.url.clone()),
            duration: info
                .length_seconds
                .parse()
                .map(Duration::from_secs)
                .unwrap_or_else(|_| {
                    warn!("Failed to parse video duration: {}", info.length_seconds);
                    Duration::from_secs(0)
                }),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum YoutubeProviderError {
    #[error("video error: {0}")]
    VideoError(rusty_ytdl::VideoError),
}
