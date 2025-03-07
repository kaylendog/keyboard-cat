mod youtube;

use songbird::input::Input;

use crate::resolver::Metadata;

/// A provider of sources.
pub trait Provider {
    // Errors that can occur when interacting with a provider.
    type Error;

    /// Returns true if a given metadata can be handled by this provider.
    fn matches(&self, url: &str) -> bool;

    /// Returns a source from a metadata.
    async fn source(&self, metadata: Metadata) -> Result<songbird::input::Input, Self::Error>;
}

pub struct GlobalProvider {
    youtube: youtube::YoutubeProvider,
    spotify: spotify::SpotifyProvider,
}

impl GlobalProvider {
    /// Initializes a new global provider.
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            youtube: youtube::YoutubeProvider,
            spotify: spotify::SpotifyProvider::new(client_id, client_secret),
        }
    }
}

impl Provider for GlobalProvider {
    type Error = GlobalProviderError;

    fn matches(&self, url: &str) -> bool {
        return true;
    }

    async fn source(&self, metadata: Metadata) -> Result<Input, Self::Error> {
        if self.youtube.matches(&metadata.url) {
            self.youtube.source(&metadata.url).await
        } else if self.spotify.matches(&metadata.url) {
            self.spotify.source(metadata).await
        } else {
            todo!()
        }
    }
}
pub enum GlobalProviderError {
    YoutubeError,
    SpotifyError(spotify::SpotifyProviderError),
}
