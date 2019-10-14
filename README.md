# AtCoder Sample Scraper

## Usage

Download an AtCoder problem page from **browser**.
This program may not work with html obtained by wget.

```
atcoder-sample-scraper HTML_FILE SAMPLE_NUM
```

```
$ cargo build --release
$ cp target/release/atcoder-sample-scraper path/to/somewhere # somewhere in $PATH

$ atcoder-sample-scraper index.html 3

$ tree cases
cases
├── in
│   ├── s1.txt
│   ├── s2.txt
│   └── s3.txt
└── out
    ├── s1.txt
    ├── s2.txt
    └── s3.txt

2 directories, 6 files
```
