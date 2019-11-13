use std::env;
use std::time::{Instant, Duration};
use std::str::FromStr;

fn string_push_approach(my_char: char, iteration_count: usize) -> (Duration, String) {
    let start = Instant::now();
    let mut my_char_string = "".to_string();

    for _i in 0..iteration_count {
        my_char_string.push(my_char);
    }

    (start.elapsed(), my_char_string)
}

fn vector_collect_approach(my_char: char, iteration_count: usize) -> (Duration, String) {
    let mut char_vector = Vec::new();
    for _i in 0..iteration_count {
        char_vector.push(my_char);
    }

    let start = Instant::now();
    let my_char_string = char_vector.iter().collect::<String>();

    (start.elapsed(), my_char_string)
}

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let iteration_count = usize::from_str(&args[0]).unwrap();

    let my_char = '🚁';

    let (duration, string) = string_push_approach(my_char, iteration_count);

    println!("String Push Approach Execution Time: {:?}\n", duration);
    drop(string);

    let (duration, _) = vector_collect_approach(my_char, iteration_count);
    
    println!("Vector Collect Approach Execution Time: {:?}", duration);
}