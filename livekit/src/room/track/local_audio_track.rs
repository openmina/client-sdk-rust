use super::{impl_track_trait, TrackShared};

#[derive(Debug)]
pub struct LocalAudioTrack {
    shared: TrackShared,
}

impl_track_trait!(LocalAudioTrack);
