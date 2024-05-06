pub mod algorithms;
pub mod commands;

use std::env;

use crate::algorithms::primality::generate_prime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("usage: ft_ssl command [command opts] [command args]");
    }
    let command = &args[1];
    
    match command.as_str() {
        "genrsa" => genrsa_command(&args[2..]),
        "rsa" => rsa_command(&args[2..]),
        "rsault" => rsault_command(&args[2..]),
        _ => panic!("Error: '{}' is an invalid command.", command)
    } 
}

fn genrsa_command(command: &[String]) {
    println!("command: {:?}", command)
}

fn rsa_command(command: &[String]) {
    println!("command: {:?}", command)
}

fn rsault_command(command: &[String]) {
    println!("command: {:?}", command)
}
