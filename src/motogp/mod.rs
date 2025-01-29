use serde::Serialize;
use crate::utils::html::{self, get_direct_text};


#[derive(Serialize)]
pub struct MotoGPRaceWeekend {
    country: String,
    round: String,
    circuit: String,
    date: String,
    
}

pub fn motogp_scraper() -> Vec<MotoGPRaceWeekend> {
    let response = reqwest::blocking::get("https://www.motogp.com/en/calendar");

    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);

    let html_round_selector = scraper::Selector::parse(".calendar-listing__event-container").unwrap();
    let html_round_items = document.select(&html_round_selector);

    let mut race_weekends: Vec<MotoGPRaceWeekend> = Vec::new();

    for html_round_item in html_round_items {

        let round =get_direct_text(&html_round_item, ".calendar-listing__status-type");
        let country = get_direct_text(&html_round_item, ".calendar-listing__title");
        let circuit = get_direct_text(&html_round_item, ".calendar-listing__location-track-name");

        let date_container = html_round_item
            .select(&scraper::Selector::parse(".calendar-listing__date-container").unwrap())
            .next()
            .unwrap();
        
        let start_day = date_container
            .select(&scraper::Selector::parse(".calendar-listing__date-start-day").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        
        let start_month = date_container
            .select(&scraper::Selector::parse(".calendar-listing__date-start-month").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        
        let end_day = date_container
            .select(&scraper::Selector::parse(".calendar-listing__date-end-day").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        
        let end_month = date_container
            .select(&scraper::Selector::parse(".calendar-listing__date-end-month").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();



        let race_weekend = MotoGPRaceWeekend {
            round: round.unwrap_or_else(|| "No round found".to_string()),
            country: country.unwrap_or_else(|| "No country found".to_string()),
            date: format!("{} {} - {} {}", start_day, start_month, end_day, end_month),
            circuit: circuit.unwrap_or_else(|| "No circuit found".to_string()),
        };

        race_weekends.push(race_weekend)
    }

    return race_weekends;
}