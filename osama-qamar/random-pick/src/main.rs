use std::env::args;

mod lib;

use lib::Command;

fn main() {
    let argument: Vec<String> = args().skip(1).collect();

    let data = Command::run("contacts.text", argument.clone()).unwrap(); 

    println!("{}",data);

    println!("Argument you entered: {:?}",argument);
}
