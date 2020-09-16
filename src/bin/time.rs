

use chrono::{ Utc, Local, DateTime };


fn main() {
    println!("{}", Utc::now());
    println!("{}", Local::now());
}