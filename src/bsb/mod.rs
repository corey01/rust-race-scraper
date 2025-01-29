use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RaceWeekend {
    round: String,
    name: String,
    dates: String,
    event_type: String
}

pub fn bsb_scraper() -> Vec<RaceWeekend>{
    let response = reqwest::blocking::get("https://www.britishsuperbike.com/calendar");

    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let html_round_selector = scraper::Selector::parse(".card").unwrap();
    let html_round_items = document.select(&html_round_selector);

    let mut race_weekends: Vec<RaceWeekend> = Vec::new();

    for html_round_item in html_round_items {
        let round = html_round_item
            .select(&scraper::Selector::parse(".header").unwrap())
            .next()
            .map(|round| round.text().collect::<String>());

        let name = html_round_item
            .select(&scraper::Selector::parse(".details h3:nth-child(1)").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());


        let dates = html_round_item
            .select(&scraper::Selector::parse(".details h3:nth-child(2)").unwrap())
            .next()
            .map(|date| date.text().collect::<String>());
            
        let event_type_option = html_round_item
            .select(&scraper::Selector::parse(".showdown-event-banner").unwrap())
            .next()
            .map(|e_type| e_type.text().collect::<String>());

        // If all values are empty, skip the row - this can happen for adverts placed in the grid
        if round.is_none() && name.is_none() && dates.is_none() && event_type_option.is_none() {
            continue;
        }

        let round = round.unwrap_or_else(|| "No round found".to_string());

        let event_type = if round == "R&G Official Test" {
            "Test".to_string() 
        } else {
            event_type_option.unwrap_or_else(|| "".to_string())
        };
        
        let race_weekend = RaceWeekend {
            round,
            name: name.unwrap_or_else(|| "No name found".to_string()),
            dates: dates.unwrap_or_else(|| "No dates found".to_string()),
            event_type
        };

        race_weekends.push(race_weekend)
    }

    return race_weekends;
}