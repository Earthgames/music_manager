use music_manager_helper::download;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();

    let mode = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Didn't get a mode");
            process::exit(1)
        }
    };

    match mode.as_str() {
        "dl" => match download(args) {
            Ok(_t) => (),
            Err(err) => {
                eprintln!("there was a error while running mode downloading: {}", err);
                process::exit(1);
            }
        },
        // "tg" => Ok(()),
        // "gn" => Ok(()),
        other => {
            eprintln!("Mode: {}, was not found", other);
            process::exit(1)
        }
    };
}
