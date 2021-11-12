use std::collections::HashMap;
use rdev::{listen, EventType};
use autopilot::key::{type_string, tap, Code, KeyCode};

fn sort_str(mut chars: Vec<char>) -> String {
    chars.sort(); 
    String::from_iter(chars)
}

fn stringify<K>(map: &HashMap<K, String>) -> String {
    sort_str(map.values().flat_map(|i| i.chars()).collect())
}

fn parse_config() -> HashMap<String, String> {
    let read = std::fs::read("config.conf").expect("no config!");
    let read = String::from_utf8_lossy(&read);
    let conf: Vec<(&str, &str)> = read
        .split("\n")
        .filter(|i| !i.is_empty())
        .map(|i| {
            let split: Vec<&str> = i.split("=").collect();
            (split.get(0).unwrap().to_owned(), split.get(1).unwrap().to_owned())
        })
        .collect();
    let mut map = HashMap::new();
    for (from, to) in conf {
        let chars = from
            .chars()
            .filter(|i| i.is_alphabetic())
            .collect();
        map.insert(sort_str(chars), to.to_string());
    }
    map
}

fn main() {
    let config = parse_config();
    let mut keys = HashMap::<String, String>::new();
    listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if event.name.is_none() { return }
                let name = event.name.unwrap();
                keys.insert(format!("{:?}", key), name);
                match config.get(&stringify(&keys)) {
                    Some(s) => {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        // TODO: find a way to backspace
                        // tap(&Code(KeyCode::Backspace), &[], 0, 0);
                        type_string(s, &[], 400., 0.);
                    },
                    None => (),
                };
            },
            EventType::KeyRelease(key) => {
                keys.remove(&format!("{:?}", key));
            }, 
            _ => (),
        }
    }).unwrap();

}
