use std::error::Error;
use std::{fs, path::PathBuf};
use csv::{Reader};
use std::collections::HashMap;
use crate::types::EmailData;

pub fn read_targets(targets_path: &PathBuf) -> Result<Vec<EmailData>, Box<dyn Error>> {
  let mut targets: Vec<EmailData> = vec![];
  
  let targets_string = fs::read_to_string(targets_path)
    .expect(&format!("Could not read file at {}", targets_path.to_str().unwrap()));
  
  let mut reader_headers = Reader::from_reader(targets_string.as_bytes());
  let headers = reader_headers.headers()?;
  let mut reader_records = Reader::from_reader(targets_string.as_bytes());
  let records = reader_records.records();
  
  for record in records {
    let record = record?;
    let mut record_values: EmailData = HashMap::new();
    
    for (index, item) in record.iter().enumerate() {
      record_values.insert(String::from(&headers[index]), String::from(item));
    }
    
    targets.push(record_values)
  }
  
  Ok(targets)
}
