use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::process::Command;

struct WordIter<'a> {
    input: &'a str,
}

impl<'a> Iterator for WordIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let start;
        let end;
        if self.input.starts_with('"') {
            start = 1;
            // find closing "
            end = self.input[1..].find('"').map(|x| x + 1);
        } else {
            start = 0;
            end = self.input.find(|c: char| c.is_whitespace());
        }

        let end = end.unwrap_or(self.input.len());
        let result = &self.input[start..end];
        // if start is > 0, skip just as much at the end
        self.input = &self.input[start + end..];
        self.input = self.input.trim_start();

        Some(result)
    }
}

fn cd(mut args: WordIter) {
    let path = match args.next() {
        Some(x) => x,
        None => {
            eprintln!("cd does not support no argument currently");
            return;
        }
    };
    if let Err(e) = std::env::set_current_dir(path) {
        eprintln!("could not change dir: {}", e);
    }
}

fn echo(args: WordIter) {
    // let's lock the stdout buffer
    // to avoid multiple implicit mutex locks
    let mut stdout = stdout().lock();

    for arg in args {
        // ignoring the error for simplicity
        // (and there's not much we can do)
        let _ = stdout.write_all(arg.as_bytes());
        let _ = stdout.write_all(b" ");
    }
    let _ = stdout.write_all(b"\n");
    let _ = stdout.flush();
}

fn exit(mut args: WordIter) {
    let code: i32 = match args.next() {
        Some(code) => match code.parse() {
            Ok(code) => code,
            Err(_e) => {
                eprintln!("exit expects integer argument");
                return;
            }
        },
        None => 0,
    };
    std::process::exit(code);
}

/// handle builtin command or spawn process
fn handle_cmd(cmd: &str, args: WordIter) {
    match cmd {
        "cd" => cd(args),
        "echo" => echo(args),
        "exit" => exit(args),
        cmd => {
            let child = Command::new(cmd).args(args).spawn();
            let mut child = match child {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("failed to launch process {}: {}", cmd, e);
                    return;
                }
            };
            let status = child.wait();
            match status {
                Ok(status) => {
                    if let Some(code) = status.code() {
                        if code != 0 {
                            eprintln!("[process terminated with exit code {}]", code);
                        }
                    } else {
                        println!("[process terminated by signal]");
                    }
                }
                Err(e) => {
                    eprintln!("[io error: {}]", e);
                }
            }
        }
    }
}

fn handle_line(line: &str) {
    let mut iter = WordIter { input: line };

    let cmd = match iter.next() {
        Some(cmd) => cmd,
        // empty lines do nothing
        None => return,
    };

    handle_cmd(cmd, iter);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to ezsh!");

    let mut buf = String::new();

    let mut stdin = stdin().lock();

    loop {
        let mut stdout = stdout().lock();
        let current_dir = std::env::current_dir()?;
        // construct a prompt from the last two segments
        // of the current working directory
        let part1 = current_dir.parent();
        let part2 = current_dir.file_name().unwrap();
        if let Some(part1) = part1 {
            stdout.write_all(part1.file_name().unwrap().as_bytes())?;
            stdout.write_all(b"/")?;
        }
        write!(stdout, "{} $ ", part2.to_str().unwrap())?;
        stdout.flush()?;
        // release lock
        drop(stdout);

        buf.clear();
        stdin.read_line(&mut buf)?;

        handle_line(buf.trim());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iter(input: &str) -> Vec<&str> {
        WordIter { input }.collect()
    }

    #[test]
    fn basic() {
        assert_eq!(iter(""), Vec::<&str>::new());
        assert_eq!(iter("asdf"), vec!["asdf"]);
        assert_eq!(iter("asdf foo"), vec!["asdf", "foo"]);
        assert_eq!(iter("asdf\t\tfoo"), vec!["asdf", "foo"]);
        assert_eq!(iter("asdf\t\tfoo   bar"), vec!["asdf", "foo", "bar"]);
    }

    #[test]
    fn quoted() {
        assert_eq!(iter("\"\""), vec![""]);
        assert_eq!(iter("\"asdf\""), vec!["asdf"]);
        assert_eq!(iter("asdf \"foo\""), vec!["asdf", "foo"]);
        assert_eq!(iter("asdf \"\" \"foo\""), vec!["asdf", "", "foo"]);
    }
}
