use std::error::Error;
use crate::types::{EmailContent, EmailData};
use std::{fs, path::PathBuf};

pub fn compose_email(target: &EmailData, html_content_path: &PathBuf, email_subject_path: &PathBuf,
) -> Result<EmailContent, Box<dyn Error>> {
  // Get email template
  // Probably to read files into memory in main, but YAGNI for now
  let mut email_html_content: String = fs::read_to_string(html_content_path).expect(&format!("Could not read file at {}", html_content_path.to_str().unwrap()));
  let mut email_subject: String = fs::read_to_string(email_subject_path).expect(&format!("Could not read file at {}", email_subject_path.to_str().unwrap()));
  
  // Replace variables on template
  // Iterate target values
  for item in target.into_iter() {
    // Curly braces are escaped doubling them.
    let key = format!("{{{{{}}}}}", item.0);
    let value = item.1;
    
    // Replace variables in source files
    email_html_content = email_html_content.replace(&key.to_lowercase(), value).trim().to_string();
    email_subject = email_subject.replace(&key.to_lowercase(), value).trim().to_string();
  }
  
  Ok(EmailContent {
    email_html_content,
    email_subject,
  })
}
