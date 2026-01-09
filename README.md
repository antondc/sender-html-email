# HTML Email sender

## Description

Sends html emails to destinations provided in CSV format

## Setup

Add connection data in an `.env` file in root folder:

```
EMAIL_HOST=mail.host.com
EMAIL_USER=hello@example.com
EMAIL_PASSWORD=string
```

Create `targets.csv` file with desired data:

- Headers must be in `snake_case` format.
- A column for `email_address` holding the destination email address must be present.

```
"name","surname","email_address"
"John","Doe","hello@example.com"
```

Create following files with desired content, adding variables existing in the provided CSV file
in `snake_case` surrounded by doubly curly braces: `{{email_address}}`:

- [./sources/email_html_content.html](./sources/email_html_content.html)
- [./sources/email_subject.txt](./sources/email_subject.txt)

## Build and Run

```
cargo build
```

For help:

```
cargo run -- -h
```

Send HTML email:

```
cargo run -- \
    --targets_path ./sources/targets.csv \
    --html_content_path ./sources/email_html_content.html \
    --email_subject_path ./sources/email_subject.txt
```

Also send with attachment —currently only .pdf files—:

```
cargo run -- \
    --targets_path ./sources/targets.csv \
    --html_content_path ./sources/email_html_content.html \
    --email_subject_path ./sources/email_subject.txt \
    --attachment_path ./sources/my_document.pdf
```

Or if you have a compiled binary:

```
 ./html-email-sender \
    --targets_path ./sources/targets.csv \
    --html_content_path ./sources/email_html_content.html \
    --email_subject_path ./sources/email_subject.txt \
    --attachment_path ./sources/my_document.pdf
 ```


## Reload

1 2
