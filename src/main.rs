extern crate afl;
extern crate lewton;

use std::io::Cursor;
use std::result::Result;

fn main() {
    afl::fuzz(|data| {
        let cursor = Cursor::new(data);

        if let Ok(mut reader) = lewton::inside_ogg::OggStreamReader::new(cursor) {
            while let Ok(Some(_)) = reader.read_dec_packet() {}
        }
    });
}
