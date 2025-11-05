use std::fs::read_to_string;
//use std::io::prelude::*;

fn main() {
    //ingest data

    let filename = "assets/everybody_codes_e2025_q01_p1.txt";

    let raw_data = match read_to_string(filename) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening and reading file: {error:?}"),
    };

    print!("{}", raw_data);

    //structure data

    //follow rules

    //print result
}

//fn structure() {}
