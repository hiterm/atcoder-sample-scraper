use std::io::prelude::*;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use scraper::{Html, Selector};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut html = String::new();
    file.read_to_string(&mut html).expect("something went wrong reading the file");

    let document = Html::parse_document(&html);

    let cases_path = Path::new("cases");
    let cases_in_path = cases_path.join("in");
    let cases_out_path = cases_path.join("out");
    fs::create_dir(cases_path).unwrap();
    fs::create_dir(cases_path.join("in")).unwrap();
    fs::create_dir(cases_path.join("out")).unwrap();

    let sample_num: usize = args[2].parse().unwrap();
    for i in 0..2*sample_num {
        let selector_prefix = "#pre-sample".to_string();
        let selector_str = selector_prefix.clone() + &i.to_string();
        let selector = Selector::parse(&selector_str).unwrap();
        let elem = document.select(&selector).next();
        match elem {
            Some(elem) => {
                println!("{}", elem.inner_html());
                let out_path;
                match i % 2 {
                    0 => out_path = cases_in_path.join(format!("s{}.txt", i/2 + 1)),
                    _ => out_path = cases_out_path.join(format!("s{}.txt", i/2 + 1)),
                }
                let mut write_file = OpenOptions::new().write(true).create_new(true).open(out_path).unwrap();
                write_file.write_all(elem.inner_html().as_bytes()).unwrap();
            }
            None => println!("Not found: {}", selector_str),
        }
    }
}
