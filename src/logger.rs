use colored::Colorize;

pub struct Logger;
impl Logger {
    pub fn info(&self, msg: &str) {
        println!("{} {}", "[INFO]: ".blue(), msg);
    }

    pub fn warn(&self, msg: &str) {
        println!("{} {}", "[Warning]: ".yellow(), msg);
    }
    pub fn error(&self, msg: &str) {
        println!("{} {}", "[ERROR]: ".red(), msg);
    }
}
