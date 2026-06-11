use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    let mut counter = 0;
    let mut the_rest: String = s;
    while the_rest.len() >= 16 {

        let tmp_rest = the_rest.split_off(16);

        print!("{:0>7}0: ", counter);


        let bytes = the_rest.as_bytes();

        for c in bytes {
            print!("{:x}", c)
        }

        print!(" ");

        for c in bytes {
            if *c >= 32 && *c <= 126 {
                print!("{}", *c as char)
            } else {
                print!(".")
            }
        }

        println!("");

        the_rest = tmp_rest;
        counter += 1;
    }
}
