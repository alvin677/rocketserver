use std::process::Command;

#[get("/api/temperature")]
pub fn get_temp() -> String {
    let temp = Command::new("vcgencmd").args(["measure_temp"]).output().unwrap();
    let temp_str = std::str::from_utf8(&temp.stdout).unwrap();
    return temp_str.to_string();
}