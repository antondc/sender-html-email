use dotenv::dotenv;
use crate::read_targets::read_targets;
use crate::send_email::send_email;
use crate::types::EmailData;
use crate::compose_email::compose_email;

mod read_targets;
mod types;
mod send_email;
mod compose_email;
mod constants;

use clap::Parser;

/// Send HTML emails via CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Path to the .csv file containing the list of targets and its variables
  #[arg(long = "targets_path")]
  targets_path: std::path::PathBuf,
  /// Path to the .html file containing the main email content
  #[arg(long = "html_content_path")]
  html_content_path: std::path::PathBuf,
  /// Path to the .txt file containing the subject
  #[arg(long = "email_subject_path")]
  email_subject_path: std::path::PathBuf,
  /// Path to the .pdf file to attach
  #[arg(long = "attachment_path")]
  attachment_path: Option<std::path::PathBuf>,
}

fn main() {
  dotenv().ok();
  
  let Args {
    html_content_path,
    email_subject_path,
    targets_path,
    attachment_path
  } = Args::parse();
  
  let targets: Vec<EmailData> = read_targets(&targets_path).expect("Error while parsing arguments");
  
  for target in targets.iter() {
    let email_content = compose_email(target, &html_content_path, &email_subject_path).expect("Error while composing email");
    
    send_email(target, email_content, &attachment_path).expect("Error on email delivery");
  }
}
