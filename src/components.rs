mod id3_tag;
pub use id3_tag::Id3v2Tag;
// mod wav_tag;
// pub use wav_tag::WavTag;
mod flac_tag;
mod mp4_tag;
pub use flac_tag::FlacTag;
pub use mp4_tag::Mp4Tag;
