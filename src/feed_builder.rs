
pub mod feed_builder {

    use chrono::{DateTime, Utc};
    use feed_rs::model;
    use feed_rs::model::{Content, Link, Text};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct FeedItem<'a> {
        id: &'a str,
        title: &'a str,
        content: &'a str,
        links: Vec<&'a str>,
        published: String
    }

    #[derive(Serialize)]
    pub struct Feed<'a> {
        id: &'a str,
        title: &'a str,
        items: Vec<FeedItem<'a>>,
    }

    fn match_for_content(title: &Option<Text>) -> &str {
        match title {
            Some(t) => { &t.content }
            None => {""}
        }
    }

    fn match_for_body(content: &Option<Content>) -> &str {
        match content {
            Some(c) => { match &c.body {
                Some(b) => { b }
                None => {""}
            }}
            None => {""}
        }
    }

    fn match_for_links(links: &Vec<Link>) -> Vec<&str> {
        let mut link_strings: Vec<&str> = Vec::new();

        for link in links {
            link_strings.push(&link.href);
        }

        link_strings
    }

    fn match_for_date(published: &Option<DateTime<Utc>>) -> String {
        match published {
            Some(t) => {
                t.date_naive().to_string()
            }
            None => { "".to_string() }
        }
    }

    // paraing broken into functions to provide good defaults
    pub fn from(parsed_feed: &model::Feed) -> Feed {
        let feed = Feed {
            id: &parsed_feed.id,
            title: match_for_content(&parsed_feed.title),
            items: parsed_feed
                .entries
                .iter()
                .map(|entry| FeedItem {
                    id: &entry.id,
                    title: match_for_content(&entry.title),
                    content: match_for_body(&entry.content),
                    published: match_for_date(&entry.published),
                    links: match_for_links(&entry.links)
                })
                .collect(),
        };

        feed
    }
}
