#[macro_use]
extern crate ini;
extern crate dirs;
use std::path::Path;

fn main() {
    let path = Path::new(dirs::home_dir().unwrap().to_str().unwrap()).join(".lcp");
    let map = ini!(path.to_str().unwrap());
    // Proceed to use normal HashMap functions on the map:
    let lcp_token = map["remote \"lcp\""]["token"].clone().unwrap();
    println!("{}", lcp_token);
}