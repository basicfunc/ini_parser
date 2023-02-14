use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

struct Section {
    name: String,
    values: Vec<HashMap<String, String>>,
}

struct Ini {
    bytes: String,
    sections: Vec<Section>,
}

impl Ini {
    fn new(bytes: String) -> Self {
        Self {
            bytes,
            sections: vec![],
        }
    }

    fn remove_comments(self) -> String {
        let mut result = String::new();

        for line in self.bytes.lines() {
            let line = line.trim();

            if let Some(idx) = line.find(';') {
                result.push_str(&line[0..idx]);
            } else if let Some(idx) = line.find('#') {
                result.push_str(&line[0..idx]);
            } else {
                result.push_str(line)
            }

            result.push('\n');
        }

        result
    }
}

fn open_ini(filename: &String) -> Result<String, std::io::Error> {
    if !filename.ends_with(".ini") {
        eprintln!("\x1B[1m\x1B[33mWarning: \"{filename}\" must end with '.ini'.\x1B[0m")
    }

    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let res = Ini::new((open_ini(&args[1])).unwrap());
        println!("{:?}", res.remove_comments());
    } else {
        eprintln!("\x1B[1m\x1B[31mError: Wrong number of argument passed!\nExpected: {:?} <filename.ini>\x1B[0m",
            args[0]
        )
    }
}
