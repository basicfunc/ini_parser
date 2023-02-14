use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

// The Ini struct is defined as a tuple struct that contains a single element of type HashMap<String, HashMap<String, String>> which will store (section_name: (key, value) pairs)
#[derive(Debug, Clone)]
struct Ini(HashMap<String, HashMap<String, String>>);

impl Ini {
    // This method will return a new instance of Ini with an empty HashMap.
    fn new() -> Self {
        Self(HashMap::new())
    }
}

// The `IniParser` struct is defined as a struct that contains a single field stream of type String which carries the source of file to be parsed.
#[derive(Debug, Clone)]
struct IniParser {
    stream: String,
}

impl IniParser {
    // This method will return a new instance of IniParser with bytes passed as argument.
    fn new(bytes: String) -> Self {
        Self { stream: bytes }
    }

    // The `remove_comments` method remove comments from the input file.
    fn remove_comments(&mut self) {
        let mut uncommented_src = String::new();

        for line in self.stream.lines() {
            let line = line.trim();

            if let Some(idx) = line.find(';') {
                uncommented_src.push_str(&line[0..idx]);
            } else if let Some(idx) = line.find('#') {
                uncommented_src.push_str(&line[0..idx]);
            } else {
                uncommented_src.push_str(line)
            }

            uncommented_src.push('\n');
        }

        self.stream = uncommented_src
    }

    // The `parse` method parses the contents of the file into the Ini data structure, and return an error if the file is not well-formed.
    fn parse(&mut self) -> Result<Ini, String> {
        // Removing Comments out of source file.
        self.remove_comments();

        // Iterating over every line of source file.
        let lines = self.stream.lines();

        // Temporary variable to hold values of current sections.
        let mut curr_section = String::new();
        let mut section: Ini = Ini::new();

        // Iterating over each line of source code.
        for (idx, line) in lines.into_iter().enumerate() {
            // Removing extra whitespaces.
            let line = line.trim();

            // Because newline and empty line don't needed to be parsed.
            if line == "\n" || line.is_empty() {
                continue;
            }
            // Identifying [section] name and instantiating Hashmap with key of name `section`.
            else if line.starts_with('[') && line.ends_with(']') {
                curr_section = line[1..line.len() - 1].trim().into();
                section.0.insert(curr_section.to_string(), HashMap::new());
            }
            // Parsing key-value pairs.
            else {
                let parts: Vec<&str> = line.split('=').collect();

                if parts.len() != 2 {
                    return Err(format!(
                        "Error: Expected \"key = value\" pair at line number {:?}.",
                        idx + 1
                    ));
                } else {
                    section
                        .0
                        .get_mut(&curr_section)
                        .unwrap()
                        .insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
        Ok(section.to_owned())
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
        let mut res = IniParser::new((open_ini(&args[1])).unwrap());
        res.remove_comments();
        let parse = res.parse().unwrap();

        println!("{:?}", parse.0.keys());
    } else {
        eprintln!("\x1B[1m\x1B[31mError: Wrong number of argument passed!\nExpected: {:?} <filename.ini>\x1B[0m",
            args[0]
        )
    }
}
