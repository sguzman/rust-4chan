extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

use reqwest::get;
use serde_json::from_str;

use chrono::prelude::{DateTime, NaiveDateTime, Utc};

#[derive(Serialize, Deserialize)]
struct Thread {
    no: u32,
    last_modified: u64
}

#[derive(Serialize, Deserialize)]
struct Page {
    page: u8,
    threads: Vec<Thread>
}

const URL: &str = "http://a.4cdn.org/sci/threads.json";

fn main() {
    let body = get(URL).unwrap().text().unwrap();
    let json: Vec<Page> = from_str(body.as_ref()).unwrap();

    let threads = {
        let mut threads = {
            let mut vec: Vec<Thread> = Vec::new();

            for p in json {
                for t in p.threads {
                    vec.push(t);
                }
            }

            vec
        };
        threads.sort_by(|a, b| a.last_modified.cmp(&b.last_modified));
        threads
    };

    for t in threads {
        let secs = {
            let secs = NaiveDateTime::from_timestamp(t.last_modified as i64, 0);
            let secs: DateTime<Utc> = DateTime::from_utc(secs, Utc);
            secs.format("%Y/%m/%d %H:%M:%S")
        };

        // Print the newly formatted date and time
        println!("{} -> {}", secs, t.no);
    }
}
