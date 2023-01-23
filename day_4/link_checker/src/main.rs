use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use thiserror::Error;
use threadpool::ThreadPool;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();
    let valid_urls = Arc::new(Mutex::new(vec![]));
    // let mut handles = vec![];
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut count = 0;
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    for element in html.select(&selector) {
        println!(
            "Sending: {}",
            element.value().attr("href").unwrap().to_string()
        );
        tx.send(element.value().attr("href").unwrap().to_string());
        count += 1;
    }
    let receiver = Arc::new(Mutex::new(rx));
    for _ in 0..count {
        let receiver = receiver.clone();
        let base_url = base_url.clone();
        let valid_urls = valid_urls.clone();
        pool.execute(move || {
            let receiver = receiver.lock().unwrap();
            let mut valid_urls: Vec<Url> = valid_urls.lock().unwrap().to_vec();
            match receiver.recv() {
                Ok(href) => {
                    let base_url = Url::parse(base_url.as_ref()).unwrap();
                    let url = base_url.join(href.as_str()).unwrap();
                    println!("{url}");
                    // valid_urls.push(url);
                }
                Err(err) => {
                    println!("Err: #{err}");
                }
            }
        });
    }
    Ok(valid_urls.clone().lock().unwrap().to_vec())
}
fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    let response = get(start_url).unwrap();
    match extract_links(response) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
