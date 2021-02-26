use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::Command;
use std::time::Instant;
use std::{fs, io};

fn main() -> Result<(), std::io::Error> {
    println!("Example: cargo_clean ~/Download/clone");

    let args: Vec<String> = env::args().collect();
    let mut counter: Vec<String> = vec![];
    let test_vec: Vec<&str> = args.iter().map(|s| s as &str).collect();
    get_result(&mut counter, &test_vec);
    println!("Data: {:?}  ", counter);
    let args_len = args.len();
    if !(2..3).contains(&args_len) {
        eprintln!("Please input a valid argument");
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Error valid arguement",
        ))
    } else {
        let entries = fs::read_dir(&args[1])?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<std::path::PathBuf>, io::Error>>()?;
        let mut child_handler = Vec::new();
        let now = Instant::now();
        for i in entries {
            let mut is_cargo: Vec<bool> = vec![];
            let path = fs::read_dir(&i)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;

            for p in path {
                if p.file_name() == Some(OsStr::new("Cargo.toml")) {
                    is_cargo.push(true);
                } else {
                    is_cargo.push(false);
                }
            }
            let handler = std::thread::spawn(move || {
                println!("Current thread: {:?}", std::thread::current().id());
                let mut clean_process = Command::new("cargo");
                clean_process.arg("clean");
                clean_process.current_dir(i);
                match clean_process.status() {
                    Ok(_) => {}
                    Err(e) => println!("Error: {:?}", e),
                }
            });
            child_handler.push(handler);
        }
        for i in child_handler {
            i.join().unwrap();
        }

        get_result(&mut counter, &test_vec);

        println!("===== Clean Size =====");
        println!("Before clean: {:?}", counter[0]);
        println!("After  clean: {:?}", counter[1]);
        println!("===== Clean time =====");
        println!("Duration    : {:?} ms", now.elapsed().as_millis());
        Ok(())
    }
}
fn get_result(reduce_size: &mut Vec<String>, args: &[&str]) {
    match exe_cmd_out("du", ["-sh", args[1]].to_vec()) {
        Ok(mut ve_out) => {
            let output = du_output(&mut ve_out);
            reduce_size.push(output);
        }
        Err(e) => eprintln!("Error; {:?}", e),
    }
}
pub fn du_output(vec_string: &mut Vec<String>) -> String {
    if let Some(da) = vec_string.pop() {
        let split: Vec<&str> = da.split('\t').collect();
        let data_output = match split.first() {
            Some(d) => d.to_string(),
            None => String::from("Nothing"),
        };
        data_output
    } else {
        String::from("Nothing")
    }
}
pub fn exe_cmd_out(program: &str, args: Vec<&str>) -> Result<Vec<String>, std::io::Error> {
    let mut single_process = Command::new(program);
    if let Ok(child_proces) = single_process.args(args).output() {
        let output = child_proces.stdout;
        let proces_out = String::from_utf8_lossy(&output);
        let result = proces_out
            .lines()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Ok(result)
    } else {
        Err(Error::new(ErrorKind::Other, "Cannot spawn a child process"))
    }
}
