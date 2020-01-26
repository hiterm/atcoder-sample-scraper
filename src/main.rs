use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

enum SampleType {
    In,
    Out,
}

impl std::fmt::Display for SampleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SampleType::In => "In",
            SampleType::Out => "Out",
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (filename, sample_total_num) = if args.len() >= 3 {
        let filename = &args[1];
        let sample_num: usize = args[2].parse().unwrap();
        (filename, sample_num)
    } else if args.len() == 2 {
        let filename = &args[1];
        (filename, 3)
    } else {
        println!("usage: command file.html [sample_num]");
        std::process::exit(1);
    };

    let mut file = File::open(filename).expect("file not found");
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect("something went wrong reading the file");

    let document = Html::parse_document(&html);

    let cases_path = Path::new("cases");
    let cases_in_path = cases_path.join("in");
    let cases_out_path = cases_path.join("out");
    fs::create_dir(cases_path).unwrap();
    fs::create_dir(cases_path.join("in")).unwrap();
    fs::create_dir(cases_path.join("out")).unwrap();

    for sample_num in 1..=sample_total_num {
        println!("### s{} ###", sample_num);
        println!("");

        for sample_type in [SampleType::In, SampleType::Out].iter() {
            let sample_index = match sample_type {
                SampleType::In => (sample_num - 1) * 2,
                SampleType::Out => (sample_num - 1) * 2 + 1,
            };

            let selector_prefix = "#pre-sample".to_string();
            let selector_str = selector_prefix.clone() + &sample_index.to_string();
            let selector = Selector::parse(&selector_str).unwrap();
            let elem = document.select(&selector).next();
            match elem {
                Some(elem) => {
                    println!("{}:", sample_type.to_string());
                    println!("{}", elem.inner_html());

                    let write_path = match sample_type {
                        SampleType::In => cases_in_path.join(format!("s{}", sample_num)),
                        SampleType::Out => cases_out_path.join(format!("s{}", sample_num)),
                    };
                    let mut write_file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(write_path)
                        .unwrap();
                    write_file.write_all(elem.inner_html().as_bytes()).unwrap();
                }
                None => println!("Not found: {}", selector_str),
            }
        }
    }
}
