use std::io;
use std::process;

fn main() {
    println!("Enter as many words as you would like and then press enter when you are done.");
    let mut words = String::new();
    io::stdin().read_line(&mut words).expect("Failed to read line");
    let words = words.trim();

    if words=="" {
        println!("No input recieved");
        process::exit(1);
    }

    let parts = words.split(" ");
    let mut num = 0;
    for _ in parts {
        num +=1;
    }
    println!("{}", num);
}
