extern crate sdl2;

use std::env;
use std::path::Path;
use sdl2::mixer::{INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG, INIT_OGG,
                 AUDIO_S16LSB};

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./demo audio.[mp3|wav|ogg]")
    } else {
        demo(Path::new(&args[1]));
    }
}

fn demo(filename: &Path) {

    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();
    let mut timer = sdl.timer().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                          INIT_MODPLUG |
                                          INIT_OGG)
                            .unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    let chunk_size = 1024;
    let _ = sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    sdl2::mixer::allocate_channels(1);

    {
        let n = sdl2::mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2::mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let chunk = sdl2::mixer::Chunk::from_file(filename).unwrap();
    let chan = sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();

    struct Effy {
        meaning: u32,
    }

    impl Drop for Effy {
        fn drop(&mut self) {
            println!("Dropped effect {}", self.meaning);
        }
    }

    impl sdl2::mixer::EffectCallback for Effy {
        type SampleType = i16; // this matches AUDIO_S16LSB for open_audio

        fn callback(&mut self, buf: &mut [i16]) {
            println!("Hello from effect {}, buf len {}: {:?} ..", self.meaning, buf.len(), &buf[..10]);
            for lr in buf.chunks_mut(2) {
                // interlaved samples; this increases volume on the left channel
                lr[0] *= 2;
            }
        }
    }

    let ef = Effy {
        meaning: 42,
    };

    // btw, SDL2_mixer removes all effects from a channel when the channel
    // is done playing, and that's when the effect is dropped, if not
    // earlier explicitly
    chan.register_effect(ef).unwrap();

    timer.delay(3000);

    println!("quitting sdl");
}
