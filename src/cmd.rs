use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Command, ExitStatus};
pub fn get_result(reduce_size: &mut Vec<String>, args: &[&str]) {
    match exe_cmd_out("du", ["-sh", args[1]].to_vec()) {
        Ok(mut ve_out) => {
            let output = du_output(&mut ve_out);
            reduce_size.push(output);
        }
        Err(e) => eprintln!("Error; {:?}", e),
    }
}
pub fn clean_cmd<P: AsRef<Path>>(
    program: &str,
    args: &[&str],
    path: P,
) -> Result<ExitStatus, Error> {
    let mut process = Command::new(program);
    process.args(args);
    process.current_dir(path);
    match process.status() {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
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
