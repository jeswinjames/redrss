use redrss::Rss;
use std::fs;

pub const CONFIG_FILE: &str = "config";

fn main() {
    let json_object = fs::read_to_string(CONFIG_FILE).expect("File Read failed");
    let rss_object = Rss::new(&json_object);
    let red_url = rss_object.url_crafter();
    let _s = redrss::request_gun(&red_url);
}
