use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Context {
    Section(String),
    Value(String, String),
    Subsection(String),
}

#[derive(Debug)]
struct Ini(Vec<Context>);

impl Ini {
    fn new() -> Self {
        Ini(Vec::new())
    }
}

fn parse(src: String) -> Result<Ini, String> {
    let lines: Vec<_> = src.lines().collect();
    let mut result = Ini::new();

    for (idx, line) in lines.iter().enumerate() {
        let context = line.trim();

        if context.chars().nth(0) == Some(';') || context.is_empty() {
            continue;
        } else if context.chars().nth(0) == Some('[')
            && context.chars().nth(context.len() - 1) == Some(']')
        {
            let section_name = &line[1..context.len() - 1];

            if section_name.chars().nth(0) == Some('.') {
                result.0.push(Context::Subsection(section_name.into()))
            } else {
                result.0.push(Context::Section(section_name.into()))
            }
        } else {
            let vals: Vec<&str> = context.split("=").collect();
            if vals.len() != 2 {
                return Err(format!(
                    "Error: expected (key, value) pair at line {}.",
                    idx + 1
                ));
            } else {
                let (key, value) = (vals[0], vals[1]);
                result.0.push(Context::Value(key.into(), value.into()))
            }
        }
    }

    return Ok(result);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let filename: String = args[1].clone();

        let ext_check = |s: &str| {
            let t = s.split(".");
            let mut t: Vec<&str> = t.collect();

            if t.pop() != Some("ini") {
                eprintln!("Warning: \"{s}\" must end with '.ini'.")
            }
        };

        ext_check(&filename);

        let mut file = File::open(filename).expect("File not found.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Unable to read content of {filename}.");

        println!("{:?}", parse(content));
    } else {
        eprintln!("Error: Wrong number of argument passed!");
        eprintln!("Expected: {:?} <filename.ini>", args[0])
    }
}
