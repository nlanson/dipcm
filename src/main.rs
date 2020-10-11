extern crate reqwest;
extern crate lettre;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::{thread, time};


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("http://api.ipify.org/").send().await?;

    // Move and borrow value of `res`
    let mut curr_ip = res.text().await?;
    println!("Your IP is:\n{}", curr_ip);

    init_mailer(&curr_ip);

    let day = time::Duration::from_secs(3600); //Change the length here to customise how often to check IP change.
    loop {
        thread::sleep(day);
        let client = reqwest::Client::new();
        let res = client.get("http://api.ipify.org/").send().await?;
        let mut new_ip = res.text().await?;
        
        if new_ip != curr_ip {
            println!("Your IP has changed! Sound the alarms!!!");
            mail_ip(&new_ip);
            curr_ip = new_ip;
        } else{
            println!("Your IP is still the same.");
            continue
        }
    }

    Ok(())
}

fn init_mailer(ip: &String) { //the function that actually sends the emails. 
    let email = Message::builder()
        .from("<<<<SENDER EMAIL HERE>>>>".parse().unwrap()) //fill
        .to("<<<<RECIPIENT EMAIL HERE>>>>".parse().unwrap()) //fill
        .subject("init ip")
        .body(ip)
        .unwrap();

    let creds = Credentials::new("<<<<SENDER EMAIL HERE>>>>".to_string(), "<<<<SENDER PASSWORD HERE>>>>".to_string()); //fill

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Init email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

fn mail_ip(ip: &String) { //the function that actually sends the emails. 
    let email = Message::builder()
        .from("<<<<SENDER EMAIL HERE>>>>".parse().unwrap())
        .to("<<<<RECIPIENT EMAIL HERE>>>>".parse().unwrap())
        .subject("Your IP has changed to:")
        .body(ip)
        .unwrap();

    let creds = Credentials::new("<<<<SENDER EMAIL HERE>>>>".to_string(), "<<<<SENDER PASSWORD HERE>>>>".to_string()); //fill

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
