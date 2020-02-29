use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Card {
    id: String,
    name: String,

    #[serde(rename(deserialize = "shortUrl"))]
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    cards: Vec<Card>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_parse_card() {
        let card_value = json!({
            "id": "trello-id-1",
            "name": "my card name",
            "shortUrl": "https://trello.com/c/thing"
        });

        let card: Card = serde_json::from_value(card_value).unwrap();

        assert_eq!(card.id, "trello-id-1");
        assert_eq!(card.name, "my card name");
        assert_eq!(card.url, "https://trello.com/c/thing");
    }

    #[test]
    fn test_search_result() {
        let search_value = json!({
            "cards": [{
                "id": "trello-id-1",
                "name": "my card name",
                "shortUrl": "https://trello.com/c/thing"
            }]
        });

        let result: SearchResult = serde_json::from_value(search_value).unwrap();

        assert_eq!(result.cards[0].id, "trello-id-1");
        assert_eq!(result.cards[0].name, "my card name");
        assert_eq!(result.cards[0].url, "https://trello.com/c/thing");
    }
}
