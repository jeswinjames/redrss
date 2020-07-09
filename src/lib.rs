use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
enum Post {
    New,
    Hot,
    Top
}

#[derive(Deserialize)]
pub struct Rss {
    reddit_secret: String,
    post_type: Post,
    no_of_post: u8,
    webhook: String
}

impl Rss {
    fn new(json_object: &str) -> Rss {
        let r: Rss = serde_json::from_str(json_object).unwrap();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample_json = r#"{
                            "reddit_secret": "Test",
                            "post_type": "Hot",
                            "no_of_post": 10,
                            "webhook": "samp"
                            }"#;

        let test_r = Rss::new(&sample_json);
        assert_eq!(test_r.reddit_secret, "Test");
    }
}
