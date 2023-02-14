use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct Section(HashMap<String, Option<Vec<HashMap<String, String>>>>);

impl Section {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Debug, Clone)]
struct Ini {
    stream: String,
    sections: Section,
}

impl Ini {
    fn new(bytes: String) -> Self {
        Self {
            stream: bytes,
            sections: Section::new(),
        }
    }

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

    fn parse(&mut self) -> Result<Section, String> {
        let lines = self.stream.lines();
        let mut section = String::new();
        let mut values: Vec<HashMap<String, String>> = vec![];

        for (idx, line) in lines.into_iter().enumerate() {
            let line = line.trim();

            if line == "\n" || line.is_empty() {
                continue;
            } else if line.starts_with('[') && line.ends_with(']') {
                if !(section.is_empty()) {
                    if values.is_empty() {
                        self.sections.0.insert(section, None);
                    } else {
                        self.sections.0.insert(section, Some(values.clone()));
                    }
                }
                section = line[1..line.len() - 1].into();
                values = vec![];
            } else {
                let parts: Vec<&str> = line.split('=').collect();

                if parts.len() != 2 {
                    return Err(format!(
                        "Error: Expected \"key = value\" pair at line number {:?}.",
                        idx + 1
                    ));
                } else {
                    let mut pairs: HashMap<String, String> = HashMap::new();
                    pairs.insert(parts[0].into(), parts[1].into());

                    values.push(pairs);
                }
            }
        }
        Ok(self.sections.to_owned())
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
        let mut res = Ini::new((open_ini(&args[1])).unwrap());
        res.remove_comments();
        let parse = res.parse().unwrap();

        println!("{:?}", parse.0.keys());
    } else {
        eprintln!("\x1B[1m\x1B[31mError: Wrong number of argument passed!\nExpected: {:?} <filename.ini>\x1B[0m",
            args[0]
        )
    }
}
