extern crate ears;
use ears::{Music, AudioController};

fn main() {
        let mut music = Music::new("test.flac").unwrap();
            music.play();


                        while music.is_playing() {};
}
