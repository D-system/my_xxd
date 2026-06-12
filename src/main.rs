use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;

fn build_hex_output(bytes: &[u8]) -> String {
    let mut result = String::new();
    
    for bytes_slice in bytes.chunks(2) {
        for c in bytes_slice {
            result.push_str(format!("{:02x}", c).as_str());
        }
        let width = 3 - bytes_slice.len();
        result.push_str(format!("{:>width$}", " ").as_str());
    }
    let width = 8 * 5 + 1; // It could be a constant of 41
    format!("{: <width$}", result)
}

fn build_file_output(bytes: &[u8]) -> String {
    let mut result = String::new();

    for c in bytes {
        if *c >= 32 && *c <= 126 {
            result.push(*c as char);
        } else {
            result.push('.');
        }
    }

    result
}

fn open_file(path: &Path) -> File {
    match File::open(path) {
        Err(why) => {
            let error = match why.kind() {
                io::ErrorKind::PermissionDenied => { "Permission denied" },
                io::ErrorKind::NotFound => { "No such file or directory" },
                _ => { "Unknown error" },
            };
            eprintln!("xxd: {}: {}", path.display(), error);
            std::process::exit(1);
        },
        Ok(file) => file,
    }
}

fn main() {
    let path = Path::new("Cargo.toml");

    let mut file = open_file(path);

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
        print!("{}", build_hex_output(&bytes));
        print!("{}", build_file_output(&bytes));

        println!("");

        the_rest = tmp_rest;
        line_counter += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_hex_output() {
        assert_eq!(build_hex_output("a".as_bytes()), "61                                       ".to_string());
        assert_eq!(build_hex_output("ab".as_bytes()), "6162                                     ".to_string());
        assert_eq!(build_hex_output("12345678".as_bytes()), "3132 3334 3536 3738                      ".to_string());
        assert_eq!(build_hex_output("1234567812345678".as_bytes()), "3132 3334 3536 3738 3132 3334 3536 3738  ".to_string());
    }
}
