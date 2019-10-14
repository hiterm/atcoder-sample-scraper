use std::fs;
use scraper::{Html, Selector};

fn main() {
    let html = fs::read_to_string("index.html").unwrap();

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
