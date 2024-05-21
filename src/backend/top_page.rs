use reqwest;
use scraper::{Html, Selector};

pub fn get_contests() -> (Vec<String>, Vec<String>, Vec<String>) {
    let selector_active =
        Selector::parse("#contest-table-action tr td:not(.text-center) a").unwrap();
    let selector_upcoming =
        Selector::parse("#contest-table-upcoming tr td:not(.text-center) a").unwrap();
    let selector_recent =
        Selector::parse("#contest-table-recent tr td:not(.text-center) a").unwrap();

    let body = reqwest::blocking::get("https://atcoder.jp/contests?lang=en")
        .unwrap()
        .text()
        .unwrap();
    let document = Html::parse_document(&body);
    let active_elements = document.select(&selector_active);
    let upcoming_elements = document.select(&selector_upcoming);
    let recent_elements = document.select(&selector_recent);

    (
        active_elements.map(|e| e.text().collect()).collect(),
        upcoming_elements.map(|e| e.text().collect()).collect(),
        recent_elements.map(|e| e.text().collect()).collect(),
    )
}
