use super::Provider;

pub struct SpotifyProvider {
    client: rspotify::ClientCredsSpotify,
}

impl SpotifyProvider {
    /// Creates a new Spotify provider.
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client: rspotify::ClientCredsSpotify::new(rspotify::Credentials::new(
                client_id,
                client_secret,
            )),
        }
    }
}

impl Provider for SpotifyProvider {
    type Error = SpotifyProviderError;

    fn matches(&self, url: &str) -> bool {
        return url.starts_with("https://open.spotify.com/");
    }

    async fn source(
        &self,
        metadata: super::Metadata,
    ) -> Result<songbird::input::Input, Self::Error> {
        return Err(SpotifyProviderError::Unsupported);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SpotifyProviderError {
    /// Unsupported operation.
    Unsupported,
}
