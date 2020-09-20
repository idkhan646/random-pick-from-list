extern crate rand;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rand::Rng;

pub enum Command {
    ListAll,
    RamdomPick,
}

impl Command {
    fn filter(arg: &Vec<String>)-> Result<Command, String>{
        if arg.is_empty() {
            return Err("Argument Is Missing".to_string());
        }
        match arg[0].to_lowercase().as_ref() {
           "listall" => Ok(Command::ListAll),
           "randompick" => Ok(Command::RamdomPick),
            _ => return Err("undefiend command".to_string()),
        }
    }

    pub fn run(file_name: &str, arg: Vec<String>) -> Result<String, String> {
        let command = Command::filter(&arg).unwrap();

        match command {
            Command::ListAll => list_all(&file_name),
            Command::RamdomPick => random_pick(&file_name),
            _ => return Err("error fatching command".to_string()),
        }
    }


}
fn random_pick(file_name: &str)->Result<String,String>{
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0,85);

    let file = read_file(file_name).unwrap();

    let buffer = BufReader::new(file);

    //initialing an empty vector to store all contacts
    let mut contact_list = Vec::new();

    //itrating over each line in the file
    for line in buffer.lines() {
        match line {
            Ok(contact) => contact_list.push(contact),
            Err(error) => return Err(format!("error reading data from file {}", error)),
        }
    };

    Ok(contact_list[num].clone())
}

fn list_all(file_name: &str)->Result<String,String>{
    let file = read_file(&file_name).unwrap();

    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    buffer.read_to_string(&mut content).unwrap();

    Ok(content)

}

pub fn read_file(name: &str)->Result<File,String>{
    let contacts = match File::open(name) {
        Ok(file) => file,
        Err(error) => return Err(format!("error opening file: {}", error))
    };
    Ok(contacts)
} 

