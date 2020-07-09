use serde::Deserialize;
use serde_json;
//use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
enum Post {
    New,
    Hot,
    Top
}

#[derive(Deserialize, Debug)]
pub struct Rss {
    subreddit: String,
    post_type: Post,
    no_of_post: u8,
    webhook: String
}

impl Rss {
    pub fn new(json_object: &str) -> Rss {
        let r: Rss = serde_json::from_str(json_object).unwrap();
        r
    }
}

pub fn url_crafter(subreddit: &str, post_type: &str, num: &str) -> String {
    let url = format!("https://reddit.com/r/{}/{}/.json?count={}", subreddit, post_type, num);
    url.to_owned()
}

//pub fn red_requester(subreddit: &str, post_type: &str, num: &str) -> String {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample_json = r#"{
                            "subreddit": "Test",
                            "post_type": "Hot",
                            "no_of_post": 10,
                            "webhook": "samp"
                            }"#;

        let test_r = Rss::new(&sample_json);
        assert_eq!(test_r.subreddit, "Test");
    }

    #[test]
    fn test_url_crafter() {
        let hardcoded_url = "https://reddit.com/r/rust/top/.json?count=1";
        let obtained_url = url_crafter("rust", "top", "1");
        assert_eq!(hardcoded_url, obtained_url);
    }
}
