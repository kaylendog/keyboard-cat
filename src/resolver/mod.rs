use std::time::Duration;

mod spotify;
mod youtube;

/// Metadata associated with a source.
pub enum Metadata {
    Single {
        title: String,
        description: String,
        image_url: Option<String>,
        duration: Duration,
        url: String,
    },
    Playlist {
        title: String,
        description: String,
        url: String,
        contents: Vec<Metadata>,
    },
}

pub trait Resolver {
    type Error;

    /// Resolves a URL or query to a metadata.
    async fn resolve(&self, url: &str) -> Result<Vec<Metadata>, Self::Error>;
}

/// A global resolver that can resolve URLs from multiple providers.
pub struct GlobalResolver {
    youtube: youtube::YoutubeResolver,
    spotify: spotify::SpotifyResolver,
}

impl GlobalResolver {
    /// Initializes a new global resolver.
    pub fn new(spotify_client_id: &str, spotify_client_secret: &str) -> Self {
        Self {
            youtube: youtube::YoutubeResolver::default(),
            spotify: spotify::SpotifyResolver::new(spotify_client_id, spotify_client_secret),
        }
    }
}

impl Resolver for GlobalResolver {
    type Error = GlobalResolverError;

    async fn resolve(&self, url: &str) -> Result<Vec<Metadata>, Self::Error> {
        // resolve all providers concurrently
        let (youtube, spotify) =
            tokio::join!(self.youtube.resolve(&url), self.youtube.resolve(&url));

        // combine results, discarding errors unless all providers fail
        let mut results = vec![];
        youtube.map(|r| results.extend(r)).unwrap_or_default();
        spotify.map(|r| results.extend(r)).unwrap_or_default();

        Ok(results)
    }
}

pub enum GlobalResolverError {
    Youtube(rusty_ytdl::VideoError),
}
