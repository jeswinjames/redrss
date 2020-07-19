use clokwerk::{Scheduler, Interval};
use chrono;
use std::thread;
use redrss::Rss;
use redrss::Rtype;
use redrss::DiscordContent;
use std::fs;
use std::time::Duration;

const CONFIG_FILE: &str = "config";

fn freq_converter(frequency: String) -> (Interval, u64) {
    let temp_list:Vec<&str> = frequency.split(' ').collect();
    let freq_num: u32 = temp_list[0].parse().expect("Enter an integer frequency");
    match &temp_list[1].to_lowercase()[..] {
        "days"    | "day"    => (Interval::Days(freq_num), freq_num as u64 * 86400_000),
        "hours"   | "hour"   => (Interval::Hours(freq_num), freq_num as u64 * 3600_000),
        "minutes" | "minute" => (Interval::Minutes(freq_num), freq_num as u64 * 60000),
        "seconds" | "second" => (Interval::Seconds(freq_num), freq_num as u64 * 1000),
        _                    => panic!("Enter a proper frequency"),
    }
}

fn runner(rss_object: &Rss) {
    println!("Revving up");
    let red_url = rss_object.url_crafter();
    let resp_str = redrss::request_gun(&red_url, Rtype::Get).unwrap();
    let red_msg = redrss::content_extractor(resp_str).unwrap();
    let disc_msg = DiscordContent::new("Heads Up! Here's the top post of the day!", red_msg);
    let disc_msg = serde_json::to_string(&disc_msg).unwrap();
    let _resp = redrss::request_gun(&rss_object.webhook, Rtype::Post(disc_msg));
}

fn main() {
    let json_object = fs::read_to_string(CONFIG_FILE).expect("File Read failed");
    let rss_object = Rss::new(&json_object);
    let (freq_enum, milliseconds) = freq_converter(rss_object.frequency.clone());
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    scheduler.every(freq_enum).at("1:00 am").run(move || runner(&rss_object));
    let _thread_handle = scheduler.watch_thread(Duration::from_millis(milliseconds));
    loop { thread::park() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_converter() {
        assert_eq!((Interval::Seconds(10),10000), freq_converter("10 Seconds".to_string()));
        assert_eq!((Interval::Minutes(22),1320_000), freq_converter("22 minuTes".to_string()));
        assert_eq!((Interval::Hours(2),7200_000), freq_converter("2 hours".to_string()));
        assert_eq!((Interval::Days(1),86400_000), freq_converter("1 day".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_freq_converter_word_freq() {
        freq_converter("one day".to_string());
    }

    #[test]
    #[should_panic]
    fn test_freq_converter_incorr_freq() {
        freq_converter("1 month".to_string());
    }
}

