use lettre::{ Message, SmtpTransport, Transport };

pub fn email() {
  // Send an email using lettre
  let email = Message::builder()
    .from("System message <no-reply@aussiebroadband.com.au>".parse().unwrap())
    .reply_to("Joshua Byrd <phocks@gmail.com>".parse().unwrap())
    .to("Joshua Byrd <byrd.joshua@proton.me>".parse().unwrap())
    .subject("A test email from Rust")
    .body(
      String::from(
        "Hi. This is just a test email. If you get it, can you please reply? And let me know if it goes to your spam folder. Thanks. Josh"
      )
    )
    .unwrap();

  // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

  // Open a remote connection to gmail
  let mailer = SmtpTransport::relay("mail.aussiebroadband.com.au")
    .unwrap()
    // .credentials(creds)
    .build();

  // Send the email
  match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {:?}", e),
  }
}