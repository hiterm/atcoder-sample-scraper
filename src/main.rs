use std::env;
use std::fs::File;
use scraper::{Html, Selector};
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut html = String::new();
    f.read_to_string(&mut html).expect("something went wrong reading the file");

    let document = Html::parse_document(&html);

    let sample_num = 3;
    for i in 0..2*sample_num {
        let selector_prefix = "#pre-sample".to_string();
        let selector_str = selector_prefix.clone() + &i.to_string();
        let selector = Selector::parse(&selector_str).unwrap();
        let elem = document.select(&selector).next();
        match elem {
            Some(elem) => println!("{}", elem.inner_html()),
            None => println!("Not found: {}", selector_str),
        }
    }
}
