use std::env;
use std::process;
use minigrep;
use minigrep::Cacher;
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

    let example_closure = |x|x;
    let s = example_closure(String::from("Hello"));
    //let n = example_closure(5); error
    //

    //let x = 4;
    let x = vec![1,2,3];
    //let equal_to_x = |z| z == x;
    let equal_to_x = move |z| z == x;

    //println!("can't use x here: {:?}", x);

    //let y = 4;

    let y = vec![1,2,3];
    assert!( equal_to_x(y));
    lterator_demo();

    


}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
  intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_closure = |num: u32| -> ui2 {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    //let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //expensive_result
            //expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            //expensive_result
            //expensive_closure(intensity)
            expensive_result.value(intensity)
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

fn lterator_demo() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val  in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]

fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

}

#[test]

fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}


