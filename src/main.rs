use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todolist {
    mapping: HashMap<String, bool>,    // Rust's builtin HashMap object, stores key and value pairs
}

impl Todolist {
    fn new() -> Result<Todolist, std::io::Error> {  // it will return a Result of wither Todolist or Error
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mapping: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todolist { mapping })
    }

    fn insert(&mut self, key: String) {
        // mut makes variable mutable - by default all variables are immutable
        // & indicates a reference (imagine variables as a pointer to the memory location where the value is stored c/f the value itself)
        self.mapping.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.mapping {  // for every key value pair, format and push the line into string
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
    // we need save to take ownership of self when it is called, so that the compiler would stop us if we accidentally update the map after saving - nifty
    // so we 'enforce' save to be the last method to be used
    // you see in this case, how Rust's memory management creates stricter code which won't compile - consequently preventing human error during development

}

fn main() {
    let action = std::env::args().nth(1).expect("What action would you like to take?");
    let item = std::env::args().nth(2).expect("What item would you like to exact this action upon?");
    // args function is from the env module of the standard library
    // args is an iterator that returns the arguments inputted (from the CLI)
    // nth is the argument at position e.g in this case 1 - we start reading from 1 because at position 0 is the program itself
    // expect function panics if value is None
    let mut todolist = Todolist {  // instationation
        mapping: HashMap::new(),
    };
    if action == "add" {
        todolist.insert(item);
        match todolist.save() {  // patternmatching
            Ok(_) => println!("Your to do list is saved!"),
            Err(why) =>  println!("An error occured while attempting to save your to do list: {}", why),
        }
    };
    // println!("{:?}, {:?}", intention, action);
}
