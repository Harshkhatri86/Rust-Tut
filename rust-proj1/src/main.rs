mod channelMultithreading;
mod hashmap;
mod lifetime;
mod multithreading;
mod traits_tut;

fn main() {
    // let x : i32 = -6 ; // i32 = signed int range negative to positive
    // let y : u32 = 1000;  // i32 = signed int  , positive range only
    // let z : f32= 100.00 ; // floating range
    println!("Hello, world!");

    let greeting: String = String::from("hello world");

    let char1 = greeting.chars().nth(1);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1"),
    }

    // calling hashmap function
    hashmap::hash_map();
    traits_tut::trt();
    lifetime::lifetime();
    multithreading::multithreading();
    channelMultithreading::channel_multithreading();
}
