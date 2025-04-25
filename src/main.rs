pub mod interpreter;



fn main() {
    let prog = String::from("+[.+]"); // Print Ascii 1~255
    // let prog = String::from(",[.,]"); // Cat

    println!("Running bf program: {}", prog);
    let prog = prog.as_bytes().to_vec();
    let result = interpreter::interp::run(&prog);
    match result {
        Ok(_) => println!("\nProgram executed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
