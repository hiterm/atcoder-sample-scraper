use scraper::{Html, Selector};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "AtCoder Sample Scraper", about = "AtCoder Sample Scraper")]
struct Opt {
    html_file: String,
    problem: String,
    #[structopt(default_value = "3")]
    case_num: usize,
}

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
    let opt = Opt::from_args();

    let filename = opt.html_file;
    let problem = opt.problem;
    let sample_total_num = opt.case_num;

    let mut file = File::open(filename).expect("file not found");
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect("something went wrong reading the file");

    let document = Html::parse_document(&html);

    let cases_root_path = Path::new("cases");
    let cases_problem_path = cases_root_path.join(problem);
    let cases_in_path = cases_problem_path.join("in");
    let cases_out_path = cases_problem_path.join("out");
    if cases_in_path.exists() || cases_out_path.exists() {
        println!(
            "Already {} or {} exists.",
            cases_in_path.display(),
            cases_out_path.display()
        );
        std::process::exit(1);
    }
    fs::create_dir_all(&cases_in_path).unwrap();
    fs::create_dir_all(&cases_out_path).unwrap();

    for sample_num in 1..=sample_total_num {
        println!("### s{} ###", sample_num);
        println!();

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
