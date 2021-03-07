pub fn show_status(counter: &Vec<String>, now: std::time::Duration) {
    println!("===== Clean Size =====");
    println!("Before clean: {:?}", counter[0]);
    println!("After  clean: {:?}", counter[1]);
    println!("===== Clean time =====");
    println!("Duration    : {:?}", now);
}
