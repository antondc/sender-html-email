use std::collections::HashMap;

pub type EmailData = HashMap<String, String>;

#[derive(Debug)]
pub struct EmailContent {
  pub email_html_content: String,
  pub email_subject: String,
}
