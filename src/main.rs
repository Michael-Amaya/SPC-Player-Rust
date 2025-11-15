mod models;

use crate::models::spc_file::SPCFile;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Usage: {} <file path>", args[0]);
    }

    let file_contents = std::fs::read(&args[1]).unwrap();

    let spc_file = SPCFile::new(file_contents).unwrap();

    // println!("Hello, world: {:?}", spc_file);
    println!("Created a SPC file!");
    print_encoded_spc_header(&spc_file);
}

fn print_encoded_spc_header(encoded_spc: &SPCFile) {
    println!("Header:\t\t{}", std::str::from_utf8(&encoded_spc.header).unwrap());
    println!("Song Title:\t\t{}", std::str::from_utf8(&encoded_spc.text_song_title).unwrap());
    println!("Game Title:\t\t{}", std::str::from_utf8(&encoded_spc.text_game_title).unwrap());
    println!("Dumper:\t\t\t{}", std::str::from_utf8(&encoded_spc.text_dumper).unwrap());
    println!("Comments:\t\t{}", std::str::from_utf8(&encoded_spc.text_comments).unwrap());
    println!("Date Dumped:\t\t{}", std::str::from_utf8(&encoded_spc.text_date_dumped).unwrap());
    println!("Fading Time:\t\t{}", std::str::from_utf8(&encoded_spc.text_num_seconds_before_fading).unwrap());
    println!("Fade Length:\t\t{}", std::str::from_utf8(&encoded_spc.text_length_fade_ms).unwrap());
    println!("Artist:\t\t\t{}", std::str::from_utf8(&encoded_spc.text_artist).unwrap());
    println!("Emulator:\t\t{}", &encoded_spc.text_emulator_used);
}