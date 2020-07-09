use redrss::Rss;
use std::fs;

pub const CONFIG_FILE: &str = "config";

fn main() {
    let json_object = fs::read_to_string(CONFIG_FILE).expect("File Read failed");
    let rss_object = Rss::new(&json_object);
    println!("----------------> {:?}", rss_object);
}
