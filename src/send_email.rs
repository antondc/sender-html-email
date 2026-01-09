use std::error::Error;
use std::env;
use std::path::{PathBuf};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport, message::{MultiPart, SinglePart}};
use crate::types::{EmailData, EmailContent};
use crate::constants::{EMAIL_ADDRESS_KEY, CONTENT_TYPE_PDF};
use std::fs;
use lettre::message::Attachment;
use lettre::message::header::ContentType;

pub fn send_email(target: &EmailData, email_content: EmailContent, attachment_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
  // Retrieve connection data from .env file
  let connection_host = env::var("CONNECTION_HOST").expect("Environment variable CONNECTION_HOST not found");
  let connection_user = env::var("CONNECTION_USER").expect("Environment variable CONNECTION_USER not found");
  let connection_password = env::var("CONNECTION_PASSWORD").expect("Environment variable CONNECTION_PASSWORD not found");
  
  let target_email_address = target.get(EMAIL_ADDRESS_KEY).expect(&format!("Could not read {} column", EMAIL_ADDRESS_KEY));
  
  // Build email with attachment only if path provided
  let multipart: MultiPart = match attachment_path {
    Some(attachment_path) => {
      let file_name = attachment_path.file_name();
      let file_body = fs::read(attachment_path).expect(&format!("Could not read file at {}", attachment_path.to_str().unwrap()));
      let file_content_type = ContentType::parse(CONTENT_TYPE_PDF).unwrap();
      let attachment = Attachment::new(file_name.unwrap().to_str().unwrap().to_owned()).body(file_body, file_content_type);
      
      MultiPart::mixed()
        .multipart(
          MultiPart::alternative()
            .multipart(
              MultiPart::related()
                .singlepart(
                  SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(String::from(&email_content.email_html_content)),
                )
                .singlepart(
                  attachment
                ),
            ),
        )
    }
    _ =>
      MultiPart::alternative()
        .singlepart(
          SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(String::from(&email_content.email_html_content)))
  };
  
  let email = Message::builder()
    .from(connection_user.parse().unwrap())
    .reply_to(connection_user.parse().unwrap())
    .to(target_email_address.parse().unwrap())
    .subject(String::from(email_content.email_subject))
    .multipart(multipart)
    .unwrap();
  
  let credentials = Credentials::new(connection_user.to_string(), connection_password.to_string());
  
  let mailer = SmtpTransport::relay(&connection_host)
    .unwrap()
    .credentials(credentials)
    .build();
  
  match mailer.send(&email) {
    Ok(_) => println!("Success: email sent successfully to {}\n", target_email_address),
    Err(e) => println!("Failure: email delivery to {} failed. \n {}\n", target_email_address, e.to_string()),
  };
  
  Ok(())
}
