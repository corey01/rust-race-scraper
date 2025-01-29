use csv::Writer;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;


// Make the function generic over any type X that implements Serialize
pub fn csv_printer<X: Serialize>(
    data: Vec<X>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(path);

    let file = File::create(file_path)?;
    // Create a CSV writer
    let mut writer = Writer::from_writer(file); // Pass the file, not the path

    // Write each record
    for record in data {
        writer.serialize(record)?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}
