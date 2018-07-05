extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
    let body = reqwest::get(URL).unwrap().text().unwrap();
    let json: Vec<Page> = serde_json::from_str(body.as_ref()).unwrap();

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
        threads.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        threads
    };

    for t in threads {
        println!("Last: {} -> Id {}", t.last_modified ,t.no);
    }
}
