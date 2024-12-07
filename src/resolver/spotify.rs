use std::time::Duration;

use rspotify::{
    model::{AlbumId, PlaylistId, TrackId},
    prelude::BaseClient,
};

use super::{Metadata, Resolver};

/// A resolver for Spotify.
///
/// Since Spotify does not provide a public API for streaming music, this resolver looks up information about the track or playlist if a URL is provided, and subsequently uses the other resolvers to find a source.
pub struct SpotifyResolver {
    client: rspotify::ClientCredsSpotify,
}

impl SpotifyResolver {
    /// Creates a new Spotify resolver.
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client: rspotify::ClientCredsSpotify::new(rspotify::Credentials::new(
                client_id,
                client_secret,
            )),
        }
    }
}

impl Resolver for SpotifyResolver {
    type Error = SpotifyResolverError;

    async fn resolve(&self, url: &str) -> Result<Vec<Metadata>, Self::Error> {
        if !url.starts_with("https://open.spotify.com/") {
            return Err(SpotifyResolverError::Unsupported);
        }

        // albums
        if url.starts_with("https://open.spotify.com/album/") {
            let album = self
                .client
                .album(
                    AlbumId::from_uri(url).map_err(SpotifyResolverError::IdError)?,
                    None,
                )
                .await?;

            return Ok(vec![Metadata::Playlist {
                title: album.name,
                description: album
                    .artists
                    .first()
                    .map(|a| a.name.clone())
                    .unwrap_or_default(),
                url: album.href,
                contents: album
                    .tracks
                    .items
                    .into_iter()
                    .filter_map(|t| t.try_into().ok())
                    .collect(),
            }]);
        }

        // playlists
        if url.starts_with("https://open.spotify.com/playlist/") {
            let playlist = self
                .client
                .playlist(
                    PlaylistId::from_uri(url).map_err(SpotifyResolverError::IdError)?,
                    None,
                    None,
                )
                .await?;

            return Ok(vec![Metadata::Playlist {
                title: playlist.name,
                description: "".to_string(),
                url: playlist.href,
                contents: playlist
                    .tracks
                    .items
                    .into_iter()
                    .filter_map(|t| t.try_into().ok())
                    .collect(),
            }]);
        }

        // tracks
        if url.starts_with("https://open.spotify.com/track/") {
            let track = self
                .client
                .track(
                    TrackId::from_uri(url).map_err(SpotifyResolverError::IdError)?,
                    None,
                )
                .await
                .map_err(SpotifyResolverError::ClientError)?;

            return Ok(vec![track.try_into()?]);
        }

        Err(SpotifyResolverError::Unsupported)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SpotifyResolverError {
    /// Unsupported operation.
    #[error("unsupported operation")]
    Unsupported,
    /// Malformed Spotify ID.
    #[error("spotify id error: {0}")]
    IdError(rspotify::model::IdError),
    /// Upstream Spotify client error.
    #[error("spotify client error: {0}")]
    ClientError(#[from] rspotify::ClientError),
    /// Missing href.
    #[error("missing href")]
    MissingHref,
}

impl TryFrom<rspotify::model::SimplifiedTrack> for Metadata {
    type Error = SpotifyResolverError;

    fn try_from(track: rspotify::model::SimplifiedTrack) -> Result<Self, Self::Error> {
        Ok(Self::Single {
            title: track.name,
            // todo: concatenate artist names
            description: "".to_string(),
            image_url: None,
            duration: Duration::from_secs(track.duration.num_seconds() as u64),
            url: track.href.ok_or(SpotifyResolverError::MissingHref)?,
        })
    }
}

impl TryFrom<rspotify::model::PlaylistItem> for Metadata {
    type Error = SpotifyResolverError;

    fn try_from(item: rspotify::model::PlaylistItem) -> Result<Self, Self::Error> {
        item.track
            .ok_or(SpotifyResolverError::Unsupported)
            .and_then(|track| track.try_into())
    }
}

impl TryFrom<rspotify::model::PlayableItem> for Metadata {
    type Error = SpotifyResolverError;

    fn try_from(item: rspotify::model::PlayableItem) -> Result<Self, Self::Error> {
        match item {
            rspotify::model::PlayableItem::Track(track) => track.try_into(),
            rspotify::model::PlayableItem::Episode(_) => Err(SpotifyResolverError::Unsupported),
        }
    }
}

impl TryFrom<rspotify::model::FullTrack> for Metadata {
    type Error = SpotifyResolverError;

    fn try_from(track: rspotify::model::FullTrack) -> Result<Self, Self::Error> {
        Ok(Self::Single {
            title: track.name,
            description: "".to_string(),
            image_url: None,
            duration: Duration::from_secs(track.duration.num_seconds() as u64),
            url: track.href.ok_or(SpotifyResolverError::MissingHref)?,
        })
    }
}
