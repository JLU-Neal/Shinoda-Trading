use chrono::NaiveDate;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;

use crate::types::Bar;

const ALPHA_VANTAGE_BASE_URL: &str = "https://www.alphavantage.co/query";

pub fn fetch_bars_from_alpha_vantage(
    api_key: &str,
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Bar>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}?function=TIME_SERIES_DAILY&symbol={}&outputsize=compact&apikey={}",
        ALPHA_VANTAGE_BASE_URL, symbol, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP request failed with status: {}", status).into());
    }

    let body: Value = response.json()?;

    if let Some(error_message) = body.get("Error Message") {
        return Err(format!(
            "Alpha Vantage API error: {}",
            error_message.as_str().unwrap_or("Unknown error")
        )
        .into());
    }

    if let Some(note) = body.get("Note") {
        return Err(format!(
            "Alpha Vantage API rate limit: {}",
            note.as_str().unwrap_or("API call frequency exceeded")
        )
        .into());
    }

    if let Some(information) = body.get("Information") {
        return Err(format!(
            "Alpha Vantage API info: {}",
            information.as_str().unwrap_or("Unknown information")
        )
        .into());
    }

    let time_series = body
        .get("Time Series (Daily)")
        .and_then(|v| v.as_object())
        .ok_or("Missing 'Time Series (Daily)' in API response")?;

    let mut sorted_entries: BTreeMap<NaiveDate, &Value> = BTreeMap::new();
    for (date_str, values) in time_series {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| format!("Failed to parse date '{}': {}", date_str, e))?;

        if date >= start_date && date <= end_date {
            sorted_entries.insert(date, values);
        }
    }

    let mut bars: Vec<Bar> = Vec::new();
    for (date, values) in &sorted_entries {
        let open = parse_price(values, "1. open", *date)?;
        let high = parse_price(values, "2. high", *date)?;
        let low = parse_price(values, "3. low", *date)?;
        let close = parse_price(values, "4. close", *date)?;
        let volume = parse_volume(values, "5. volume", *date)?;

        bars.push(Bar {
            date: *date,
            symbol: symbol.to_string(),
            open,
            high,
            low,
            close,
            volume,
        });
    }

    Ok(bars)
}

fn parse_price(values: &Value, key: &str, date: NaiveDate) -> Result<f64, Box<dyn std::error::Error>> {
    let price_str = values
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing '{}' for date {}", key, date))?;

    let price: f64 = price_str
        .parse()
        .map_err(|e| format!("Failed to parse '{}' value '{}' for date {}: {}", key, price_str, date, e))?;

    if price <= 0.0 {
        return Err(format!("Invalid price for '{}' at date {}: {}", key, date, price).into());
    }

    Ok(price)
}

fn parse_volume(values: &Value, key: &str, date: NaiveDate) -> Result<f64, Box<dyn std::error::Error>> {
    let volume_str = values
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing '{}' for date {}", key, date))?;

    let volume: f64 = volume_str
        .parse()
        .map_err(|e| format!("Failed to parse '{}' value '{}' for date {}: {}", key, volume_str, date, e))?;

    Ok(volume)
}

pub fn save_bars_to_csv(bars: &[Bar], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(output_path)
        .map_err(|e| format!("Failed to create file '{}': {}", output_path, e))?;

    writeln!(file, "date,open,high,low,close,volume")?;

    for bar in bars {
        writeln!(
            file,
            "{},{},{},{},{},{}",
            bar.date.format("%Y-%m-%d"),
            bar.open,
            bar.high,
            bar.low,
            bar.close,
            bar.volume
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_bars_from_csv;

    #[test]
    fn test_parse_alpha_vantage_json() {
        let json_str = r#"{
            "Time Series (Daily)": {
                "2024-01-02": {
                    "1. open": "150.00",
                    "2. high": "155.00",
                    "3. low": "149.00",
                    "4. close": "153.00",
                    "5. volume": "1000000"
                },
                "2024-01-03": {
                    "1. open": "153.00",
                    "2. high": "158.00",
                    "3. low": "152.00",
                    "4. close": "157.00",
                    "5. volume": "1200000"
                },
                "2024-01-04": {
                    "1. open": "157.00",
                    "2. high": "160.00",
                    "3. low": "155.00",
                    "4. close": "159.00",
                    "5. volume": "900000"
                }
            }
        }"#;

        let body: Value = serde_json::from_str(json_str).unwrap();
        let time_series = body.get("Time Series (Daily)").unwrap().as_object().unwrap();

        let start_date = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap();

        let mut sorted_entries: BTreeMap<NaiveDate, &Value> = BTreeMap::new();
        for (date_str, values) in time_series {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            if date >= start_date && date <= end_date {
                sorted_entries.insert(date, values);
            }
        }

        assert_eq!(sorted_entries.len(), 2);

        let first_date = *sorted_entries.keys().next().unwrap();
        assert_eq!(first_date, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());

        let first_values = sorted_entries.values().next().unwrap();
        let open = parse_price(first_values, "1. open", first_date).unwrap();
        assert_eq!(open, 150.0);
    }

    #[test]
    fn test_date_range_filter() {
        let json_str = r#"{
            "Time Series (Daily)": {
                "2024-01-01": {
                    "1. open": "100.00",
                    "2. high": "105.00",
                    "3. low": "99.00",
                    "4. close": "104.00",
                    "5. volume": "500000"
                },
                "2024-01-15": {
                    "1. open": "110.00",
                    "2. high": "115.00",
                    "3. low": "109.00",
                    "4. close": "114.00",
                    "5. volume": "600000"
                },
                "2024-02-01": {
                    "1. open": "120.00",
                    "2. high": "125.00",
                    "3. low": "119.00",
                    "4. close": "124.00",
                    "5. volume": "700000"
                }
            }
        }"#;

        let body: Value = serde_json::from_str(json_str).unwrap();
        let time_series = body.get("Time Series (Daily)").unwrap().as_object().unwrap();

        let start_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        let mut filtered: BTreeMap<NaiveDate, &Value> = BTreeMap::new();
        for (date_str, values) in time_series {
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            if date >= start_date && date <= end_date {
                filtered.insert(date, values);
            }
        }

        assert_eq!(filtered.len(), 1);
        assert!(filtered.contains_key(&NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
    }

    #[test]
    fn test_save_and_reload_csv() {
        let bars = vec![
            Bar {
                date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
                symbol: "TEST".to_string(),
                open: 150.0,
                high: 155.0,
                low: 149.0,
                close: 153.0,
                volume: 1000000.0,
            },
            Bar {
                date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                symbol: "TEST".to_string(),
                open: 153.0,
                high: 158.0,
                low: 152.0,
                close: 157.0,
                volume: 1200000.0,
            },
        ];

        let temp_path = format!("temp_fetcher_test_{}.csv", std::process::id());
        save_bars_to_csv(&bars, &temp_path).unwrap();

        let loaded_bars = load_bars_from_csv(&temp_path, "TEST").unwrap();
        assert_eq!(loaded_bars.len(), 2);
        assert_eq!(loaded_bars[0].open, 150.0);
        assert_eq!(loaded_bars[0].close, 153.0);
        assert_eq!(loaded_bars[1].open, 153.0);
        assert_eq!(loaded_bars[1].volume, 1200000.0);

        let _ = std::fs::remove_file(&temp_path);
    }

    #[test]
    fn test_api_error_message_detection() {
        let json_str = r#"{
            "Error Message": "Invalid API call. Please retry or visit the documentation."
        }"#;

        let body: Value = serde_json::from_str(json_str).unwrap();

        let has_error = body.get("Error Message").is_some();
        assert!(has_error);
    }

    #[test]
    fn test_api_rate_limit_detection() {
        let json_str = r#"{
            "Note": "Thank you for using Alpha Vantage! Our standard API rate limit is 25 requests per day."
        }"#;

        let body: Value = serde_json::from_str(json_str).unwrap();

        let has_note = body.get("Note").is_some();
        assert!(has_note);
    }
}
