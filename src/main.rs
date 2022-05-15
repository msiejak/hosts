use std::env;
use std::fs;
use whoami;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1: &str = &args[1][..];
    match arg1 {
        "add"=>add(args),
        "list"=>list(),
        _ => help()
    }
}

fn add(args: Vec<String>){
    if whoami::username() != "root" {
        eprintln!("This command must be run with root privileges");
        std::process::exit(1);
    }
    let ip: &str = &args[2][..];
    let alias: &str = &args[3][..];
    let new_text = get_file_txt() + "\n" + alias + " " + ip;
    fs::write("/etc/hosts", new_text).expect("error writing");
}

fn get_file_txt() -> String {
    return fs::read_to_string("/etc/hosts")
        .expect("Something went wrong reading the file");
}

fn help() {
    println!("commands are 'add [alias] [ip]' and 'list'")
}

fn list() {
    let contents = get_file_txt();
    println!("{}", contents);
}
