use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

use crate::types::Bar;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn load_bars_from_csv(path: &str, symbol: &str) -> Result<Vec<Bar>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    let mut records: Vec<CsvRecord> = Vec::new();
    for result in csv_reader.deserialize() {
        let record: CsvRecord = result?;
        records.push(record);
    }

    if records.is_empty() {
        return Err("CSV file contains no data".into());
    }

    let mut bars: Vec<Bar> = Vec::new();
    let mut previous_date: Option<chrono::NaiveDate> = None;

    for record in records {
        if record.open <= 0.0 {
            return Err(format!("Invalid open price: {} at date {}", record.open, record.date).into());
        }
        if record.high <= 0.0 {
            return Err(format!("Invalid high price: {} at date {}", record.high, record.date).into());
        }
        if record.low <= 0.0 {
            return Err(format!("Invalid low price: {} at date {}", record.low, record.date).into());
        }
        if record.close <= 0.0 {
            return Err(format!("Invalid close price: {} at date {}", record.close, record.date).into());
        }

        let date = chrono::NaiveDate::parse_from_str(&record.date, "%Y-%m-%d")
            .map_err(|e| format!("Failed to parse date '{}': {}", record.date, e))?;

        if let Some(prev) = previous_date {
            if date <= prev {
                return Err(format!(
                    "Dates are not in ascending order. Date {} comes after {}",
                    record.date, prev
                )
                .into());
            }
        }
        previous_date = Some(date);

        bars.push(Bar {
            date,
            symbol: symbol.to_string(),
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            volume: record.volume,
        });
    }

    Ok(bars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn create_temp_csv(content: &str) -> String {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_path = format!("temp_test_{}_{}.csv", std::process::id(), count);
        let mut file = File::create(&temp_path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        temp_path
    }

    fn cleanup_temp_csv(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_load_valid_csv() {
        let csv_content = "date,open,high,low,close,volume\n\
            2024-01-01,100.0,105.0,99.0,104.0,1000.0\n\
            2024-01-02,104.0,108.0,103.0,107.0,1200.0\n\
            2024-01-03,107.0,110.0,106.0,109.0,1500.0";

        let temp_path = create_temp_csv(csv_content);

        let result = load_bars_from_csv(&temp_path, "AAPL");
        assert!(result.is_ok(), "Failed to load valid CSV: {:?}", result.err());

        let bars = result.unwrap();
        assert_eq!(bars.len(), 3);
        assert_eq!(bars[0].symbol, "AAPL");
        assert_eq!(bars[0].open, 100.0);
        assert_eq!(bars[0].close, 104.0);

        cleanup_temp_csv(&temp_path);
    }

    #[test]
    fn test_load_empty_csv() {
        let csv_content = "date,open,high,low,close,volume";

        let temp_path = create_temp_csv(csv_content);

        let result = load_bars_from_csv(&temp_path, "AAPL");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no data"));

        cleanup_temp_csv(&temp_path);
    }

    #[test]
    fn test_load_invalid_price() {
        let csv_content = "date,open,high,low,close,volume\n\
            2024-01-01,100.0,105.0,99.0,104.0,1000.0\n\
            2024-01-02,-5.0,108.0,103.0,107.0,1200.0";

        let temp_path = create_temp_csv(csv_content);

        let result = load_bars_from_csv(&temp_path, "AAPL");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid open price"));

        cleanup_temp_csv(&temp_path);
    }
}
