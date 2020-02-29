use crate::trello::SearchResult;
use std::env;

#[macro_use]
extern crate log;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod trello;

fn main() {
    env_logger::init();
    let result = fetch_trello_search().unwrap();
    println!("{:#?}", result);
}

fn fetch_trello_search() -> Result<SearchResult> {
    let key = env::var("TRELLO_KEY").expect("TRELLO_KEY environment variable not set");
    let token = env::var("TRELLO_TOKEN").expect("TRELLO_TOKEN environment variable not set");

    debug!("Making the request");

    let url_raw = format!("https://api.trello.com/1/search?query=@me&key={key}&token={token}", key = key, token = token);

    let result = reqwest::blocking::get(&url_raw)?
        .json::<trello::SearchResult>()?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_thing() {
        assert!(true);
    }
}
