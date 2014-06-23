#![crate_id = "dsp"]
#![deny(missing_doc)]
#![feature(macro_rules)]
#![feature(phase)]

//! A pure Rust audio digital signal processing library for Piston.

extern crate portaudio;
extern crate time;

pub use SoundStream = sound_stream::SoundStream;
pub use SoundStreamSettings = sound_stream_settings::SoundStreamSettings;

mod port_audio_back_end;
mod sound_stream;
mod sound_stream_settings;
mod node;

