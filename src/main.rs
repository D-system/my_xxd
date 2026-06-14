use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, io};

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

fn open_file(path: &Path) -> Option<File> {
    match File::open(path) {
        Err(why) => {
            let error = match why.kind() {
                io::ErrorKind::PermissionDenied => { "Permission denied" },
                io::ErrorKind::NotFound => { "No such file or directory" },
                _ => { "Unknown error" },
            };
            eprintln!("xxd: {}: {}", path.display(), error);
            None
        },
        Ok(file) => Some(file),
    }
}

fn main() {
    let args = env::args();
    
    if args.len() != 2 {
        eprintln!("This xxd version requires a filename to output");
        std::process::exit(1);
    }

    let path = args.last().unwrap();
    let path = Path::new(path.as_str());

    let Some(mut file) = open_file(path) else {
        std::process::exit(1);
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
    use std::{fs, os::unix::fs::PermissionsExt};
    use tempfile::NamedTempFile;

    #[test]
    fn test_build_hex_output() {
        assert_eq!(build_hex_output("a".as_bytes()), "61                                       ");
        assert_eq!(build_hex_output("ab".as_bytes()), "6162                                     ");
        assert_eq!(build_hex_output("12345678".as_bytes()), "3132 3334 3536 3738                      ");
        assert_eq!(build_hex_output("1234567812345678".as_bytes()), "3132 3334 3536 3738 3132 3334 3536 3738  ");
    }

    #[test]
    fn test_build_file_output() {
        assert_eq!(build_file_output("a".as_bytes()), "a");
        assert_eq!(build_file_output("a b".as_bytes()), "a b");
        assert_eq!(build_file_output("a\nb".as_bytes()), "a.b");
        assert_eq!(build_file_output("a\tc".as_bytes()), "a.c");

        // Checking the length of a Japanese character to double check expected output
        let jp_char = "あ";
        assert_eq!(jp_char.as_bytes().len(), 3);
        assert_eq!(build_file_output(jp_char.as_bytes()), "...");
    }

    #[test]
    fn test_open_file_exiting_file() {
        let tmp = NamedTempFile::new().unwrap();
        assert!(open_file(tmp.path()).is_some());
    }

    #[test]
    fn test_open_file_missing_file() {
        assert!(open_file(Path::new("./none_existing_file")).is_none());
    }

    #[test]
    fn test_open_file_missing_permission() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path();

        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(path, permissions).unwrap();

        // It's the same assertion as the missing file. It's good enough as it check it does not continue.
        assert!(open_file(path).is_none());
    }
}
