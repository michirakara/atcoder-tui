use reqwest;
use scraper::{Html, Selector};

pub fn get_problems(contest_path: &str) -> (Vec<String>, Vec<String>) {
    let selector = Selector::parse("tbody a").unwrap();
    let body = reqwest::blocking::get(format!(
        "https://atcoder.jp/contests/{}/tasks",
        contest_path
    ))
    .unwrap()
    .text()
    .unwrap();

    let document = Html::parse_document(&body);
    let element = document.select(&selector);

    let tmp: Vec<String> = element.clone().map(|e| e.text().collect()).collect();
    let tmp_url: Vec<String> = element
        .map(|e| e.attr("href").into_iter().collect())
        .collect();
    let mut to_ret: Vec<String> = Vec::new();
    let mut to_ret_url: Vec<String> = Vec::new();
    for i in (0..tmp.len()).step_by(2) {
        to_ret.push(format!("{} - {}", tmp[i], tmp[i + 1]));
        to_ret_url.push(format!("https://atcoder.jp{}", tmp_url[i]));
    }

    (to_ret, to_ret_url)
}
