use std::env;
use std::process;
use minigrep;
use minigrep::Config;
use std::time::Duration;
use std::thread;



fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    //run(config);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1)
    }

    simulated_expensive_calculation(10);



}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
  intensity
}
