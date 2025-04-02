use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use clap::{arg, Parser};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'l')]
    line_count: bool,

    #[arg(short = 'b')]
    bytes_count: bool,

    //#[arg(trailing_var_args = true)]
    paths: Vec<PathBuf>,
}

impl Cli {
    // First milestone: count the number of line one file has
    fn run_single_file(&self, path: PathBuf) {
        println!("processing {}", path.display());
        let file_result = File::open(path.clone());
        let mut f;
        match file_result {
            Ok(file) => {f = file;}
            Err(e) => {
                println!("{} {}", path.display(), e);
                return
            }
        }

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
                        if self.line_count == true && bytes[idx] == 10 {
                            line_count += 1;
                        }
                    }
                },
                Err(e) => {
                    panic!("error happened {}", e)
                },
            }
        }
        //println!("path is {}", path.display());
        println!("{} {}", line_count, path.display());
    }

    fn run(&self) {
        for path in self.paths.clone() {
            self.run_single_file(path);
        }
    }
}

fn main() {
    let cli = Cli::parse();
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
        let mut cmd = Command::cargo_bin("wc").unwrap();
        cmd.arg("-l").arg("resource/test.in").arg("resource/test_one_line").arg("resource/test_empty")
            .assert()
            .success()
            .stdout(contains("10 resource/test.in"))
            .stdout(contains("0 resource/test_empty"))
            .stdout(contains("1 resource/test_one_line"));
    }
}
