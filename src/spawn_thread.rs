use super::cmd::clean_cmd;
use super::helper::get_dirs;
use std::path::Path;
pub fn dir_thread<P: AsRef<Path>>(p: P) {
    match get_dirs(p) {
        Ok(entries) => {
            let mut child_handler = Vec::new();
            for i in entries {
                let handler = std::thread::spawn(move || match clean_cmd("cargo", &["clean"], i) {
                    Ok(s) => println!("Status: {}", s),
                    Err(e) => eprintln!("Error: {:?}", e),
                });
                child_handler.push(handler);
            }
            for i in child_handler {
                i.join().unwrap();
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
