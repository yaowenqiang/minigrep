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
    
    let simulated_user_specified_value = 10;
    let simplated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simplated_random_number
    );
    //simulated_expensive_calculation(10);
    //expensive_closure(10)


}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
  intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    //let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //expensive_result
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            //expensive_result
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
               // expensive_result
            );
        }
    } 


}
