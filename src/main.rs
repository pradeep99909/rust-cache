mod cache;
mod config;
use cache::data::Data;
use config::operation::Type;
use std::io;

fn main() {
    let mut data: Data = Data::new();
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let args: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        let operations: Type = Type::to_int(&args[0]);
        match operations {
            Type::READ => {
                let key: &String = &args[1];
                data.read(key.trim().to_string());
            }
            Type::WRITE => {
                if args[1].is_empty() || args[2].is_empty() {
                    print!("Parameter missing");
                    return;
                }
                let operation = args[0].clone();
                let key: String = args[1].clone();
                let val: String = args[2].trim().parse().expect("value required");
                data.write(operation, key, val)
            }
        }
    }
}
