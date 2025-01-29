use csv_printer::csv_printer;

mod motogp;
mod wsbk;
mod bsb;
mod csv_printer;
mod utils;

fn main() {
    let wsbk_races = wsbk::wsbk_scraper();
    let bsb_races = bsb::bsb_scraper();
    let motogp_races = motogp::motogp_scraper();

    csv_printer(wsbk_races, "output/wsbk.csv").unwrap();
    csv_printer(bsb_races, "output/bsb.csv").unwrap();
    csv_printer(motogp_races, "output/motogp.csv").unwrap();
}

