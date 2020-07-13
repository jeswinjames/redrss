use redrss::Rss;
use redrss::Rtype;
use redrss::DiscordContent;
use std::fs;

const CONFIG_FILE: &str = "config";

fn main() {
    let json_object = fs::read_to_string(CONFIG_FILE).expect("File Read failed");
    let rss_object = Rss::new(&json_object);
    let red_url = rss_object.url_crafter();
    let resp_str = redrss::request_gun(&red_url, Rtype::Get).unwrap();
    let red_msg = redrss::content_extractor(resp_str).unwrap();
    let disc_msg = DiscordContent::new("Test", red_msg);
    let disc_msg = serde_json::to_string(&disc_msg).unwrap();
    let _resp = redrss::request_gun(&rss_object.webhook, Rtype::Post(disc_msg));
}
