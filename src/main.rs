use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use std::thread;
use redrss::Rss;
use redrss::Rtype;
use redrss::DiscordContent;
use std::fs;
use std::time::Duration;

const CONFIG_FILE: &str = "config";

fn runner(json_object: String) {
    let rss_object = Rss::new(&json_object);
    let red_url = rss_object.url_crafter();
    let resp_str = redrss::request_gun(&red_url, Rtype::Get).unwrap();
    let red_msg = redrss::content_extractor(resp_str).unwrap();
    let disc_msg = DiscordContent::new("Test", red_msg);
    let disc_msg = serde_json::to_string(&disc_msg).unwrap();
    let _resp = redrss::request_gun(&rss_object.webhook, Rtype::Post(disc_msg));
}

fn test_function() {
    println!("TEST");
}

fn main() {
    //let json_object = fs::read_to_string(CONFIG_FILE).expect("File Read failed");
    let mut scheduler = Scheduler::new();
    scheduler.every(1.minutes()).run(|| test_function());//(move || runner(json_object));
    let thread_handle = scheduler.watch_thread(Duration::from_millis(60030));
    thread_handle.stop();
}
