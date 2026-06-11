use std::fs::File;
use std::io::Read;
use std::path::Path;

fn print_hex(bytes: &[u8]) {
    let mut result = String::new();
    
    for bytes_slice in bytes.chunks(2) {
        for c in bytes_slice {
            result.push_str(format!("{:02x}", c).as_str());
        }
        let width = 3 - bytes_slice.len();
        result.push_str(format!("{:>width$}", " ").as_str());
    }
    let width = 8 * 5 + 1; // It could be a constant of 41
    print!("{: <width$}", result);
}

fn print_file(bytes: &[u8]) {
    let mut result = String::new();

    for c in bytes {
        if *c >= 32 && *c <= 126 {
            result.push(*c as char);
        } else {
            result.push('.');
        }
    }

    print!("{}", result)
}

fn main() {
    let path = Path::new("Cargo.toml");

    let mut file = match File::open(path) {
        Err(why) => panic!("Does not exists: {} {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Something went wrong: {}", why),
        Ok(_size) => {},
    };

    let mut line_counter = 0;
    let mut the_rest: String = s;
    while the_rest.len() >= 1 {
        let tmp_rest = the_rest.split_off(the_rest.len().min(16));

        print!("{:0>7}0: ", line_counter);

        let bytes = the_rest.as_bytes();
        print_hex(&bytes);
        print_file(&bytes);

        println!("");

        the_rest = tmp_rest;
        line_counter += 1;
    }
}
