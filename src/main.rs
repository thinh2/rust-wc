use std::io::{Error, Read};
use std::path::PathBuf;
use std::fs::File;

struct Cli {
    cmd_type: String,
    path: std::path::PathBuf,
}

impl Cli {
    fn new(cmd_type: String, path: String) -> Self {
        Self { cmd_type,  path: PathBuf::from(path) }
    }

    // First milestone: count the number of line one file has
    fn run(self) {
        // Here you would implement the logic to handle the command type and path
        // For example, you could read a file at the given path or perform some action based on cmd_type
        println!("Running command: {} on path: {:?}", self.cmd_type, self.path);
        let mut f = File::open(self.path.clone()).ok().unwrap();

        let mut bytes : [u8; 1000] = [0; 1000];
        let mut line_count = 0;
        loop {
            let read_result=  f.read(&mut bytes[..]);
            match read_result {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    for idx in 0..n {
                        if bytes[idx] == 10 {
                            line_count += 1;
                        }
                    }
                },
                Err(e) => {
                    panic!("error happened {}", e)
                },
            }
        }
        println!("{} {}", line_count, self.path.display());
    }
}

fn main() {
    let cmd_type = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2).unwrap();

    let cli = Cli::new(cmd_type.clone(), path.clone());
    cli.run();
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    #[test]
    fn test_one_line() {
        let mut cmd = Command::cargo_bin("wc").unwrap();
        cmd.arg("-l").arg("resource/test_one_line")
            .assert()
            .success()
            .stdout(contains("1 resource/test_one_line"));
    }

    #[test]
    fn test_empty_file() {
        let mut cmd = Command::cargo_bin("wc").unwrap();
        cmd.arg("-l").arg("resource/test_empty")
            .assert()
            .success()
            .stdout(contains("0 resource/test_empty"));
    }

    #[test]
    fn test_multiple_line() {
        let mut cmd = Command::cargo_bin("wc").unwrap();
        cmd.arg("-l").arg("resource/test.in")
            .assert()
            .success()
            .stdout(contains("10 resource/test.in"));
    }

    #[test]
    fn test_multiple_file() {

    }
}
