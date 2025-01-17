use cxx::UniquePtr;

use crate::video_frame::ffi::VideoFrame;

#[cxx::bridge(namespace = "livekit")]
pub mod ffi {

    #[derive(Debug)]
    #[repr(i32)]
    pub enum TrackState {
        Live,
        Ended,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum ContentHint {
        None,
        Fluid,
        Detailed,
        Text,
    }

    // -1 = optional
    pub struct VideoTrackSourceConstraints {
        pub min_fps: f64,
        pub max_fps: f64,
    }

    unsafe extern "C++" {
        include!("livekit/media_stream.h");
        include!("livekit/video_frame.h");

        type NativeVideoFrameSink;
        type MediaStreamTrack;
        type MediaStream;
        type AudioTrack;
        type VideoTrack;
        type VideoFrame = crate::video_frame::ffi::VideoFrame;

        fn id(self: &MediaStream) -> String;

        fn kind(self: &MediaStreamTrack) -> String;
        fn id(self: &MediaStreamTrack) -> String;
        fn enabled(self: &MediaStreamTrack) -> bool;
        fn set_enabled(self: Pin<&mut MediaStreamTrack>, enable: bool) -> bool;
        fn state(self: &MediaStreamTrack) -> TrackState;

        unsafe fn add_sink(self: Pin<&mut VideoTrack>, sink: Pin<&mut NativeVideoFrameSink>);
        unsafe fn remove_sink(self: Pin<&mut VideoTrack>, sink: Pin<&mut NativeVideoFrameSink>);

        fn set_should_receive(self: Pin<&mut VideoTrack>, should_receive: bool);
        fn should_receive(self: &VideoTrack) -> bool;
        fn content_hint(self: &VideoTrack) -> ContentHint;
        fn set_content_hint(self: Pin<&mut VideoTrack>, hint: ContentHint);

        fn create_native_video_frame_sink(
            observer: Box<VideoFrameSinkWrapper>,
        ) -> UniquePtr<NativeVideoFrameSink>;

        unsafe fn video_to_media(track: *const VideoTrack) -> *const MediaStreamTrack;
        unsafe fn audio_to_media(track: *const AudioTrack) -> *const MediaStreamTrack;
        unsafe fn media_to_video(track: *const MediaStreamTrack) -> *const VideoTrack;
        unsafe fn media_to_audio(track: *const MediaStreamTrack) -> *const AudioTrack;

        fn _unique_media_stream_track() -> UniquePtr<MediaStreamTrack>; // Ignore
        fn _unique_media_stream() -> UniquePtr<MediaStream>; // Ignore
        fn _unique_audio_track() -> UniquePtr<AudioTrack>; // Ignore
        fn _unique_video_track() -> UniquePtr<VideoTrack>; // Ignore
    }

    extern "Rust" {
        type VideoFrameSinkWrapper;

        fn on_frame(self: &VideoFrameSinkWrapper, frame: UniquePtr<VideoFrame>);
        fn on_discarded_frame(self: &VideoFrameSinkWrapper);
        fn on_constraints_changed(
            self: &VideoFrameSinkWrapper,
            constraints: VideoTrackSourceConstraints,
        );
    }
}

unsafe impl Sync for ffi::MediaStreamTrack {}
unsafe impl Send for ffi::MediaStreamTrack {}
unsafe impl Sync for ffi::MediaStream {}
unsafe impl Send for ffi::MediaStream {}
unsafe impl Send for ffi::AudioTrack {}
unsafe impl Sync for ffi::AudioTrack {}
unsafe impl Send for ffi::VideoTrack {}
unsafe impl Sync for ffi::VideoTrack {}
unsafe impl Send for ffi::NativeVideoFrameSink {}
unsafe impl Sync for ffi::NativeVideoFrameSink {}

pub trait VideoFrameSink: Send + Sync {
    fn on_frame(&self, frame: UniquePtr<VideoFrame>);
    fn on_discarded_frame(&self);
    fn on_constraints_changed(&self, constraints: ffi::VideoTrackSourceConstraints);
}

pub struct VideoFrameSinkWrapper {
    observer: *mut dyn VideoFrameSink,
}

impl VideoFrameSinkWrapper {
    /// # Safety
    /// VideoFrameSink must lives as long as VideoSinkInterfaceWrapper does
    pub unsafe fn new(observer: *mut dyn VideoFrameSink) -> Self {
        Self { observer }
    }

    fn on_frame(&self, frame: UniquePtr<VideoFrame>) {
        unsafe {
            (*self.observer).on_frame(frame);
        }
    }

    fn on_discarded_frame(&self) {
        unsafe {
            (*self.observer).on_discarded_frame();
        }
    }

    fn on_constraints_changed(&self, constraints: ffi::VideoTrackSourceConstraints) {
        unsafe {
            (*self.observer).on_constraints_changed(constraints);
        }
    }
}
