use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RaceWeekend {
    round: String,
    name: String,
    dates: String,
    url: String
}

const BASE_URL: &str = "https://www.worldsbk.com"; 

pub fn wsbk_scraper() -> Vec<RaceWeekend>{
    let response = reqwest::blocking::get("https://www.worldsbk.com/en/calendar");

    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let html_calendar_selector = scraper::Selector::parse(".circuit_calendar > li").unwrap();
    let html_calendar_items = document.select(&html_calendar_selector);

    let mut race_weekends: Vec<RaceWeekend> = Vec::new();

    for html_calendar_item in html_calendar_items {
        let url = html_calendar_item
            .select(&scraper::Selector::parse("a.track-link").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(|url| format!("{}{}", BASE_URL, url));

        let round = html_calendar_item
            .select(&scraper::Selector::parse(".event-data > .round").unwrap())
            .next()
            .map(|round| round.text().collect::<String>());

        let name = html_calendar_item
            .select(&scraper::Selector::parse(".event-data > h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
            

        let dates = html_calendar_item
            .select(&scraper::Selector::parse(".date").unwrap())
            .next()
            .map(|date| date.text().collect::<String>());
            

        // If all values are empty, skip the row - this can happen for adverts placed in the grid
        if round.is_none() && name.is_none() && dates.is_none() && url.is_none() {
            continue;
        }
        
        let race_weekend = RaceWeekend {
            round: round.unwrap_or_else(|| "No round found".to_string()),
            name: name.unwrap_or_else(|| "No name found".to_string()),
            dates: dates.unwrap_or_else(|| "No dates found".to_string()),
            url: url.unwrap_or_else(|| "No URL found".to_string())
        };

        race_weekends.push(race_weekend)
    }

    return race_weekends;
}