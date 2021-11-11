use std::collections::HashMap;
use rdev::{listen, EventType};
use enigo::*;

fn sort_str(mut chars: Vec<char>) -> String {
    chars.sort();
    String::from_iter(chars)
}

fn stringify<K>(map: &HashMap<K, String>) -> String {
    sort_str(map.values().flat_map(|i| i.chars()).collect())
}

fn parse_config() -> HashMap<String, String> {
    let read = std::fs::read("config.toml").expect("no config!");
    let read = String::from_utf8_lossy(&read);
    let conf: toml::value::Table = toml::from_str(&read).unwrap();
    let mut map = HashMap::new();
    for (from, to) in conf {
        let chars = from
            .to_string()
            .as_str()
            .chars()
            .filter(|i| i.is_alphabetic())
            .collect();
        map.insert(sort_str(chars), to.to_string());
    }
    map
}

fn main() {
    let config = parse_config();
    let mut enigo = Enigo::new();
    let mut keys = HashMap::<String, String>::new();
    listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if event.name.is_none() { return }
                let name = event.name.unwrap();
                keys.insert(format!("{:?}", key), name);
                match config.get(&stringify(&keys)) {
                    Some(s) => { enigo.key_sequence(s) },
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
