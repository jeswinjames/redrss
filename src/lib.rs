use serde::Deserialize;
use serde_json;
use reqwest;

#[derive(Deserialize, Debug)]
pub struct Rss {
    subreddit: String,
    post_type: String,
    no_of_post: u8,
    webhook: String
}

impl Rss {
    pub fn new(json_object: &str) -> Rss {
        let r: Rss = serde_json::from_str(json_object).unwrap();
        r
    }

    pub fn url_crafter(&self) -> String {
    let url = format!("https://reddit.com/r/{}/{}/.json?count={}", self.subreddit, self.post_type.to_lowercase(), self.no_of_post);
    url.to_owned()
    }
}

pub fn request_gun(url: &str) -> Result<String, &'static str> {
    let custom_ua = "redrss-bot/0.1.0 reqwest/0.10.6 (by /u/n01syspy)";
    let client = reqwest::blocking::Client::builder().user_agent(custom_ua).build().expect("Client Building failed");
    let res = client.get(url).send().expect("Error in firing");
    if res.status().as_u16() != 200 {
        return Err("Got 404 error");
    }
    Ok(res.text().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample_json = r#"{
                            "subreddit": "Test",
                            "post_type": "hot",
                            "no_of_post": 10,
                            "webhook": "samp"
                            }"#;

        let test_r = Rss::new(&sample_json);
        assert_eq!(test_r.subreddit, "Test");
    }

    #[test]
    fn test_url_crafter() {
        let hardcoded_url = "https://reddit.com/r/rust/top/.json?count=1";
        let rss_object = Rss {
                        subreddit: String::from("rust"),
                        post_type: String::from("Top"),
                        no_of_post: 1,
                        webhook: String::from("test")};
        let obtained_url = rss_object.url_crafter();
        assert_eq!(hardcoded_url, obtained_url);
    }

    #[test]
    fn test_request_gun() {
        let url = "https://google.com";
        assert!(request_gun(url).is_ok());
    }
}
