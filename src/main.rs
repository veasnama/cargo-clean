use std::env;
use std::time::Instant;
mod cargo_check;
mod cmd;
mod helper;
mod spawn_thread;
mod status;
use cmd::get_result;
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut counter: Vec<String> = vec![];
    let test_vec: Vec<&str> = args.iter().map(|s| s as &str).collect();
    get_result(&mut counter, &test_vec);
    let args_len = args.len();
    let now = Instant::now();
    if !(2..3).contains(&args_len) {
        eprintln!("Please input a valid argument");
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Error valid arguement",
        ))
    } else {
        spawn_thread::dir_thread(&args[1]);
        get_result(&mut counter, &test_vec);
        status::show_status(&counter, now.elapsed());
        Ok(())
    }
}
