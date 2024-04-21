use std::env;


fn main() {
    for argument in env::args() {
        if argument == "--help" {
            println!("You passed --help as one of the arguments!");
        }
    }

    let arguments: Vec<String> = env::args().collect();
    println!("{:?}", arguments);
    
    println!("{} arguments passed", arguments.len()-1);
}