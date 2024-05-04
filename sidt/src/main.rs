use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let action = &args[1];

    let text = &args[1..];

    let entry = text.join(" ");

    println!("{:?}",entry);
}