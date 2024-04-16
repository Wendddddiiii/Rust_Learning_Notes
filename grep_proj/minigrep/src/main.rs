use std::env; // Corrected import
use std::fs;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // Use the `Config::build` method and handle the Result with `unwrap_or_else`
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

    // fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //     let contents = fs::read_to_string(config.file_path)?;

    //     println!("With text:\n{contents}");

    //     Ok(())
    // }

