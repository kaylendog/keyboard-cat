use std::time::Duration;

use rusty_ytdl::search::SearchResult;

use super::{Metadata, Resolver};

/// A resolver for YouTube.
#[derive(Debug)]
pub struct YoutubeResolver {
    api: rusty_ytdl::search::YouTube,
}

impl Default for YoutubeResolver {
    fn default() -> Self {
        Self {
            api: rusty_ytdl::search::YouTube::new().expect("failed to initialise rusty_ytdl"),
        }
    }
}

impl Resolver for YoutubeResolver {
    type Error = rusty_ytdl::VideoError;

    async fn resolve(&self, url: &str) -> Result<Vec<Metadata>, Self::Error> {
        let result = self
            .api
            .search(
                url,
                Some(&rusty_ytdl::search::SearchOptions {
                    limit: 25,
                    safe_search: false,
                    search_type: rusty_ytdl::search::SearchType::All,
                }),
            )
            .await?;

        Ok(result
            .into_iter()
            .filter(|r| !matches!(r, SearchResult::Channel(_)))
            .map(|r| r.into())
            .collect())
    }
}

impl From<rusty_ytdl::search::SearchResult> for Metadata {
    fn from(result: rusty_ytdl::search::SearchResult) -> Self {
        match result {
            SearchResult::Video(video) => Self::Single {
                title: video.title,
                description: video.description,
                image_url: video.thumbnails.first().map(|t| t.url.clone()),
                duration: Duration::from_secs(video.duration),
                url: video.url,
            },
            SearchResult::Playlist(playlist) => Self::Playlist {
                title: playlist.name,
                description: "".to_string(),
                url: playlist.url,
                contents: playlist.videos.into_iter().map(|v| v.into()).collect(),
            },
            _ => unreachable!(),
        }
    }
}

impl From<rusty_ytdl::search::Video> for Metadata {
    fn from(video: rusty_ytdl::search::Video) -> Self {
        Self::Single {
            title: video.title,
            description: video.description,
            image_url: video.thumbnails.first().map(|t| t.url.clone()),
            duration: Duration::from_secs(video.duration),
            url: video.url,
        }
    }
}
