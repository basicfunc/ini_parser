//! The code provided is a Rust module that implements a minimal Ini parser
//! which can parse almost all stable features of INI file other than
//! subsections as described by Wikipedia.

//! The module defines two functions, `parse_ini` and `parse_ini_file`, which
//! take in an Ini file's content as a string and a filename respectively and
//! return a Result containing either the parsed Ini file or an error message
//! in case if the file is not well-formed.

//! The module contains a Ini struct, defined as a tuple struct that contains
//! a single element of type `HashMap<String, HashMap<String, String>>`, which
//! will store (section_name: (key, value) pairs). The struct also implements
//! the Display trait for pretty printing.

//! The module contains a IniParser struct, which contains a single field
//! stream of type String that carries the source of the file to be parsed.
//! The IniParser struct implements two methods, remove_comments and parse,
//! which are responsible for removing comments from the input file and
//! parsing the contents of the file into the Ini data structure respectively.

//! The parse method returns an error if the file is not well-formed.

//! The module also contains two macros, `warn` and `error`, which are used to
//! print warnings in bold-yellow text messages and return error messages
//! which are formatted in bold-red text respectively.

// Imports from std library.
use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::io::{ErrorKind, Read};

// Macro to print warnings directly on stdout.
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("\x1B[1m\x1B[33m{}\x1B[0m", format!($($arg)*))
    }
}

// Macro to return errors.
macro_rules! error {
    ($($arg:tt)*) => (
        format!("\x1B[1m\x1B[31m{}\x1B[0m", format!($($arg)*))
    )
}

// The Ini struct is defined as a tuple struct that contains a single element of type HashMap<String, HashMap<String, String>> which will store (section_name: (key, value) pairs)
#[derive(Debug, Clone)]
pub struct Ini(HashMap<String, HashMap<String, String>>);

impl std::fmt::Display for Ini {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (section_name, section) in &self.0 {
            writeln!(f, "[{}]", section_name)?;
            for (key, value) in section {
                writeln!(f, "{} = {}", key, value)?;
            }
        }
        Ok(())
    }
}

impl Ini {
    // This method will return a new instance of Ini with an empty HashMap.
    fn new() -> Self {
        Self(HashMap::new())
    }

    // The `section` method returns a reference to a `HashMap` containing the key-value pairs of the section_name. If the section does not exist, an empty `HashMap` will be returned.
    pub fn section(&self, section_name: &str) -> HashMap<String, String> {
        match self.0.get(section_name) {
            Some(v) => v.clone(),
            None => HashMap::new(),
        }
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

            // Skiping part starting from ';' till EoL.
            if let Some(idx) = line.find(';') {
                uncommented_src.push_str(&line[0..idx]);
            }
            // Skiping part starting from '#' till EoL.
            else if let Some(idx) = line.find('#') {
                uncommented_src.push_str(&line[0..idx]);
            }
            // Don't need to skip.
            else {
                uncommented_src.push_str(line)
            }

            // End of Line.
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
                // Splitting by '='.
                let parts: Vec<&str> = line.split('=').collect();

                // As only key and value are expected.
                if parts.len() != 2 {
                    return Err(error!(
                        "Error: Expected \"key = value\" pair at line number {:?}.",
                        idx + 1
                    ));
                }
                // Making a pair and inserting into section.
                else {
                    let pair = match section.0.entry(curr_section.clone()) {
                        Entry::Occupied(occupied) => occupied.into_mut(),
                        Entry::Vacant(_) => {
                            return Err(error!("{curr_section} section doesn't exists."));
                        }
                    };

                    pair.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
        Ok(section.to_owned())
    }
}

// Parses the provided INI-formatted string `src` and returns a Result containing either an `Ini` struct representing the parsed INI data, or an error message as a `String`.
pub fn parse_ini(src: String) -> Result<Ini, String> {
    let mut parser = IniParser::new(src);
    parser.remove_comments();
    Ok(parser.parse().unwrap())
}

// Parses the provided INI-file of name `filename` and returns a Result containing either an `Ini` struct representing the parsed INI data, or an error message as a `String`.
pub fn parse_ini_file(filename: &String) -> Result<Ini, String> {
    if !filename.ends_with(".ini") {
        warn!("Warning: \"{filename}\" must end with '.ini'.");
    }

    let mut file;

    match File::open(filename) {
        Ok(f) => file = f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err(error!("Unable to open {filename}: not found."));
            } else if e.kind() == ErrorKind::PermissionDenied {
                return Err(error!("Unable to open {filename}: permission denied."));
            } else {
                return Err(error!("Unable to open {filename}: unknown error occured."));
            }
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => parse_ini(content),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(error!("error specified file does not exist."))
            } else if e.kind() == ErrorKind::PermissionDenied {
                Err(error!(
                    "The current user does not have permission to access the specified file."
                ))
            } else if e.kind() == ErrorKind::Interrupted {
                Err(error!(
                    " The read operation was interrupted by another signal."
                ))
            } else if e.kind() == ErrorKind::UnexpectedEof {
                Err(error!(
                    "An unexpected end of file was encountered during the read operation."
                ))
            } else {
                Err(error!("Unepected error occured while reading from specified file, there are many reasons of this error such as invalid data in the file, disk errors, or insufficient memory."))
            }
        }
    }
}
