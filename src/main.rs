use std::io::{stdin, stdout, Write};

fn main() {
    let mut store: Vec<KeyValuePair> = vec![];
    let mut _exit = false;

    while !_exit {
        let mut input = String::new();
        print!("> ");
        stdout().flush();
        stdin().read_line(&mut input);

        input = input.trim().to_string();

        if input == "exit".to_string() {
            _exit = true;
            print!("Memory Srver Exited");
        } else if input.starts_with("set ") && input.contains(" ") {
            let data: Vec<&str> = input.split(" ").to_owned().collect();

            let store_data = KeyValuePair {
                Key: data[1].to_string(),
                Value: data[2].to_string(),
            };

            store.append(&mut vec![store_data]);
            println!("OK");

        } else if input.starts_with("getall") {
            for item in &store {
                println!("{}:{}", item.Key, item.Value);
            }
        } else if input.starts_with("get ") {
            let data: Vec<&str> = input.split(" ").to_owned().collect();

            for kv in &store {
                if kv.Key == data[1] {
                    println!("{}", kv.Value);
                    break;
                } else {
                    println!("Key Not Exist");
                }
            }
        } else if input == "clear".to_string() {
            store.clear();
            println!("Store Cleared");
        } else if input.starts_with("drop ") {
            let data: Vec<&str> = input.split(" ").to_owned().collect();

            for kv in &store {
                if kv.Key == data[1] {
                    let index = store.iter().position(|x| *x.Key == kv.Key).unwrap();
                    store.remove(index);
                    println!("OK");
                    break;
                } else {
                    println!("Key Not Exist");
                }
            }
        } else {
           
        }
    }
}

struct KeyValuePair {
    Key: String,
    Value: String,
}
