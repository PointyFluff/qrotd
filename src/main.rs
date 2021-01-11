use qrcode::QrCode;
use image::Luma;
use structopt::StructOpt;
use std::time::SystemTime;
use std::fs;

// how should this app behave?
// run as a daemon or one time
// accept user command line input (gonna need z clap, yah).
// output errors to stderr, or just panic.
// output a unixtimestamp.png (svg? zoidberg?)
// Image generation could include options for 
// different borders / backgrounds
// and perhaps text
// maybe be systemd friendly. maybe.
// maybe a cowsay option. but with a qr bubble? 
// naw too big...just text
// in daemon mode we are gonna have to keep track of the day.

#[derive(StructOpt)]
struct Options {
    message: String,
}


fn main() {
    let options = Options::from_args();
    let fortunes_file = String::from("/usr/share/fortunes");
    let data = fs::read(fortunes_file).expect("unable to read file");
    let mut fortunes: Vec<Vec<u8>> = Vec::new();
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let file_name = timestamp.to_string() + ".png";
    let code = QrCode::new(options.message).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(file_name).unwrap();
}
