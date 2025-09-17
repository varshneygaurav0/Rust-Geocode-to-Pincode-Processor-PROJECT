use calamine::{open_workbook, Reader, Xlsx};
use std::error::Error;
use reqwest::blocking::Client;
use serde::Deserialize;
use xlsxwriter::*;

#[derive(Deserialize)]
struct NominatimResponse {
    address: Option<Address>,
}

#[derive(Deserialize)]
struct Address {
    postcode: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input/Input_one.xlsx";
    println!("Opening Excel file: {}", path);
    
    // Check if file exists
    if !std::path::Path::new(path).exists() {
        eprintln!("ERROR: Input file not found at path: {}", path);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found")));
    }

    println!("File exists, attempting to open...");
    // Open Excel
    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(wb) => {
            println!("Successfully opened workbook");
            wb
        },
        Err(e) => {
            eprintln!("Error opening workbook: {}", e);
            return Err(Box::new(e));
        }
    };
    
    println!("Accessing worksheet 'Sheet1'");
    let range = match workbook.worksheet_range("Sheet1") {
        None => {
            eprintln!("ERROR: Sheet1 not found in workbook");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Sheet1 not found")));
        }
        Some(Ok(range)) => {
            println!("Successfully accessed Sheet1");
            range
        }
        Some(Err(e)) => {
            eprintln!("Error reading worksheet: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Setting up HTTP client and preparing to process rows");
    let client = Client::new();
    let mut results: Vec<(String, String)> = Vec::new();

    let rows: Vec<_> = range.rows().skip(1).collect();
    println!("Found {} rows to process", rows.len());

    for (index, row) in rows.iter().enumerate() {
        if row.len() < 2 {
            eprintln!("ERROR: Row {} has insufficient columns (expected 2, got {})", index + 1, row.len());
            continue;
        }
        
        let outlet_code = row[0].to_string();
        let geocode = row[1].to_string();
        println!("Processing row {}: Outlet Code = {}, Geocode = {}", index + 1, outlet_code, geocode);
        
        let parts: Vec<&str> = geocode.split(',').collect();
        if parts.len() == 2 {
            let lat = parts[0].trim();
            let lon = parts[1].trim();

            let url = format!(
                "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json",
                lat, lon
            );

            println!("Fetching pincode for {}...", outlet_code);
            let res: NominatimResponse = client
                .get(&url)
                .header("User-Agent", "RustGeocodeApp")
                .send()?
                .json()?;

            let pincode = res
                .address
                .and_then(|a| a.postcode)
                .unwrap_or("Not Found".to_string());

            results.push((outlet_code.clone(), pincode));
        }
    }

    println!("Processed {} records, writing to output file", results.len());
    
    // Ensure output directory exists
    std::fs::create_dir_all("output").map_err(|e| {
        eprintln!("ERROR: Failed to create output directory: {}", e);
        e
    })?;
    
    // Write results to new Excel
    println!("Creating output workbook at output/pincode_output.xlsx");
    let mut workbook_out = match Workbook::new("output/pincode_output.xlsx") {
        Ok(wb) => {
            println!("Successfully created output workbook");
            wb
        },
        Err(e) => {
            eprintln!("ERROR: Failed to create output workbook: {}", e);
            return Err(Box::new(e));
        }
    };
    
    println!("Adding worksheet");
    let mut sheet = match workbook_out.add_worksheet(None) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Failed to add worksheet: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Writing headers");
    sheet.write_string(0, 0, "Outlet Code", None)?;
    sheet.write_string(0, 1, "Pincode", None)?;

    for (i, (code, pin)) in results.iter().enumerate() {
        sheet.write_string((i + 1) as u32, 0, code, None)?;
        sheet.write_string((i + 1) as u32, 1, pin, None)?;
    }

    workbook_out.close()?;
    println!("Output written to output/pincode_output.xlsx");

    Ok(())
}