use std::sync::Mutex;

lazy_static! {
    //static ref OUTFILE: String = "index.csv".to_string();
    pub static ref LOGGABLE_MTX: Mutex<i8> = Mutex::new(0);
}

pub trait Loggable {
    fn log(str: &str);
    fn warn(str: &str);
    fn err(str: &str);
}
