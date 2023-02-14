extern crate ini_parser;

use ini_parser::parse_ini_file;

fn main() {
    let res = parse_ini_file("sample.ini").unwrap();

    println!("{:?}", res.section("INSTALL").get("ALLUSERS"));
}
