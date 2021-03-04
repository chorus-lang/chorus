use std::{
    env,
    fs::{read_dir, File},
    io::*,
    sync::*,
    thread,
    time,
};

use clap::{App, Arg};
use colored::*;

mod parser;

fn main() {
    let matches = App::new("Chorus")
        .name("chorus")
        .version("0.0.1")
        .author("krista-chan")
        .about("Stupid idea by stupid girl")
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("The .chr file to compile from")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("lang")
                .value_name("language")
                .required(true)
                .takes_value(true)
                .possible_values(&["ts", "typescript"])
                .help("Select the compile target here\n"),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .takes_value(true)
                .help("Specify an output file to write compile code to"),
        )
        .get_matches();

    match matches.value_of("lang").unwrap() {
        "ts" | "typescript" => compile(
            String::from(matches.value_of("INPUT_FILE").unwrap()),
            String::from(matches.value_of("lang").unwrap()),
        ),
        _ => println!(
            "{} '{}' is not a valid compile target.",
            "Error:".red().bold(),
            matches.value_of("lang").unwrap().yellow()
        ),
    }
}

fn compile(input_file: String, lang: String) {
    const MSGS: [&str; 4] = ["[.]", "[·]", "[˙]", "[·]"];
    let is_done = Arc::new(Mutex::new(false));
    let mut readfile_buffer = String::new();

    let done_clone = is_done.clone();

    if !read_dir(env::current_dir().unwrap()).unwrap().any(|file| {
        file.unwrap().file_name() == std::ffi::OsString::from(&input_file)
    }) {
        println!(
            "{} Unable to find that file in this directory.",
            "Error:".red().bold()
        );
        std::process::exit(0);
    }

    File::open(input_file)
        .expect("Unable to open specified file.")
        .read_to_string(&mut readfile_buffer)
        .unwrap();

    let now = Arc::new(Mutex::new(time::Instant::now()));
    let elapsed: Arc<Mutex<u128>> = Arc::new(Mutex::new(0));
    let e_clone = Arc::clone(&elapsed);

    std::thread::spawn(move || {
        parser::Parser::new(parser::Languages::Typescript, readfile_buffer)
            .parse();

        *is_done.lock().unwrap() = true; /* will end compile msg loop after
                                            compilation */
        *elapsed.lock().unwrap() = now.lock().unwrap().elapsed().as_millis();
    });

    loop {
        for msg in MSGS.iter() {
            print!("Compiling to {} ", lang.yellow());
            print!("{}\r", msg);
            stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }
        if *done_clone.lock().unwrap() {
            break;
        } else {
            continue;
        };
    }
    println!("{} {}ms", "Finished compiling in".green(), e_clone.lock().unwrap())
}
