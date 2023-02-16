# ini_parser

INI Parser is a Rust library for parsing INI files, which are commonly used to store configuration data. This library provides a simple and easy-to-use interface for working with INI files.

## Overview

This is a minimal INI parser written in Rust which can parse almost all stable features of INI files other than subsections. The module provides two functions: `parse_ini` and `parse_ini_file`, which take in an INI file's content as a string and a filename respectively and return a Result containing either the parsed INI file or an error message in case the file is not well-formed.

## Introduction

INI files are a simple and widely-used format for storing configuration data. They consist of sections, each containing key-value pairs. The sections are denoted by a name in square brackets, and the key-value pairs are separated by an equals sign (=).

This library makes it easy to work with INI files in Rust. You can read and write INI files, as well as modify the data in them.

## Usage
The module defines an `Ini` struct, which contains a single element of type `HashMap<String, HashMap<String, String>>`, that will store `(section_name: (key, value) pairs)`.

The module contains an `IniParser` struct, which contains a single field stream of type String that carries the source of the file to be parsed. The IniParser struct has method `parse`, which is responsible for parsing the contents of the file into the `Ini` data structure. The parse method returns an error if the file is not well-formed.


To use it in your project, just add it to your `Cargo.toml`:
```
[dependencies]
ini-parser = "0.1.0"
```

It can be used as follows:

```
extern crate ini_parser;
use ini_parser::{parse_ini_file, Ini};

fn main() {
    let ini_content = r#"
        [section1]
        key1=value1
        key2=value2
        [section2]
        key3=value3
        key4=value4
    "#;

    let ini = parse_ini_file("example.ini").unwrap();
    let section1 = ini.section("section1").get("key1");
    println!("{section1}")
    let section2 = ini.section("section2").get("key3");
    println!("{section1}")
}

```

**This parser does not yet support INI subsections, soon it will.**

## [License](https://opensource.org/license/mit/)

Copyright (c) 2022 Rahul Sharma

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
