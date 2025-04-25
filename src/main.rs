use std::io::Read;
use std::fs::File;

pub mod interpreter;

fn main() {
    let filename = "bf/helloworld.bf";
    println!("Running bf program: {}", filename);
    let prog = File::open(filename)
        .expect("Failed to open file")
        .bytes()
        .filter_map(Result::ok)
        .collect::<Vec<u8>>();
    let result = interpreter::interp::run(&prog);
    match result {
        Ok(_) => println!("\nProgram executed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
