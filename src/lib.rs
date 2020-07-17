use serde::{Serialize, Deserialize};
use serde_json::{Value};
use reqwest::header::{CONTENT_TYPE, USER_AGENT};

pub enum Rtype {
    Get,
    Post(String)
}

#[derive(Deserialize, Debug)]
pub struct Rss {
    subreddit: String,
    post_type: String,
    no_of_post: u8,
    pub webhook: String,
    pub frequency: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Embeds {
    title: String,
    #[serde(rename(deserialize = "selftext"))]
    description: String,
    //TODO CHANGE from url to permalink as some urls have image links and not post ones
    //Proper way is to concatenate reddit.com with permalink
    url: String,
    #[serde(rename(serialize = "type"))]
    m_type: Option<String>
}

impl Embeds {
    fn mutate(&mut self) {
        self.m_type = Some("rich".to_owned());
    }
}

#[derive(Serialize, Debug)]
pub struct DiscordContent {
    content: String,
    embeds: Vec<Embeds>
}

impl DiscordContent {
    pub fn new(msg_contents: &str, embeds: Embeds) -> DiscordContent {
        DiscordContent {
            content: msg_contents.to_owned(),
            embeds: vec!(embeds)
        }
    }
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

pub fn request_gun(url: &str, r_type: Rtype) -> Result<String, &'static str> {
    let custom_ua = "redrss-bot/0.1.0 reqwest/0.10.6 (by /u/n01syspy)";
    let client = reqwest::blocking::Client::new();
    let res = match r_type {
        Rtype::Get => client.get(url)
                            .header(USER_AGENT, custom_ua)
                            .send().expect("Error in firing"),
        Rtype::Post(data) => client.post(url).body(data)
                                   .header(CONTENT_TYPE, "application/json")
                                   .header(USER_AGENT, custom_ua)
                                   .send().expect("Error in firing")};
    if res.status().as_u16() == 400 {
        //TODO COVER A RANGE OF HTTP ERRORS INSTEAD OF ONLY 404
        return Err("Got 404 error");
    }
    Ok(res.text().unwrap())
}

pub fn content_extractor(response_string: String) -> Result<Embeds,()> {
    let loose_json_object: Value = serde_json::from_str(&response_string).unwrap();
    let mut v: Embeds = serde_json::from_value(loose_json_object["data"]["children"][0]["data"].to_owned()).unwrap();
    v.mutate();
    Ok(v)
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
                            "webhook": "samp",
                            "frequency": "1 hour"
                            }"#;

        let test_r = Rss::new(&sample_json);
        assert_eq!(test_r.subreddit, "Test");
        let embed = Embeds {
                    title: String::from("Test"),
                    description: String::from("Test Description"),
                    url: String::from("test url"),
                    m_type: Some(String::from("rich"))
                    };
        let test_r = DiscordContent::new("teeeest", embed);
        assert_eq!(test_r.embeds[0].m_type, Some(String::from("rich")));
    }

    #[test]
    fn test_url_crafter() {
        let hardcoded_url = "https://reddit.com/r/rust/top/.json?count=1";
        let rss_object = Rss {
                        subreddit: String::from("rust"),
                        post_type: String::from("Top"),
                        no_of_post: 1,
                        webhook: String::from("test"),
                        frequency: String::from("1 hour")};
        let obtained_url = rss_object.url_crafter();
        assert_eq!(hardcoded_url, obtained_url);
    }

    #[test]
    fn test_request_gun() {
        let url = "https://google.com";
        assert!(request_gun(url, Rtype::Get).is_ok());
    }

    #[test]
    fn test_content_extractor() {
        let dummy_string = r#"{
                             "data": {
                               "children": [{
                                           "data": {
                                                  "title": "test_title",
                                                  "selftext": "test descriptION",
                                                  "url": "dummy_url"
                                                    }
                                           }]
                                    }
                              }"#;
        assert!(content_extractor(dummy_string.to_owned()).is_ok());
    }
}
