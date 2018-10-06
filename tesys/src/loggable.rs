use std::sync::Mutex;

extern crate colored;
pub use self::colored::*;

lazy_static! {
    //static ref OUTFILE: String = "index.csv".to_string();
    pub static ref LOGGABLE_MTX: Mutex<i8> = Mutex::new(0);
}

pub trait Loggable {
    fn log(str: &str);
    fn warn(str: &str);
    fn err(str: &str);
}

#[allow(dead_code)]
pub fn log(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Info".magenta().bold(), str.green())
    );
}

#[allow(dead_code)]
pub fn warn(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Warning".magenta().bold(), str.yellow())
    );
}

#[allow(dead_code)]
pub fn err(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Error".magenta().bold(), str.red())
    );
}
