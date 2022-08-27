use std::{fmt::format, io};
use std::io::Write;
use hyper::{Client, Response, Body};
use hyper_tls::HttpsConnector;
use json::JsonValue;
use rand::{thread_rng, Rng};
use::text_io::scan;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();

    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = String::from(format!("https://restcountries.com/v3.1/all/"));
    println!("parsing url..");
    let uri = url.parse()?;

    println!("Sending request..");
    let resp = client.get(uri).await?;

    println!("Getting response..");
    let response_content_json = body_to_string(resp).await;

    println!("Parsing response..");
    let parsed = json::parse(&response_content_json).unwrap();
        
    println!("Ready to go.");
    print!("Enter number of countries to guess: ");
    io::stdout().flush().unwrap();
    let choice : i32;
    scan!("{}",choice);
    guess(choice, &parsed);
    Ok(())
}

fn guess(n : i32,parsed: &JsonValue) {
    let mut number_of_guessed_right = 0;
    for _ in 0..n {
        
        let current_country_index = thread_rng().gen_range(0..249);
        
        let current_country = &parsed[current_country_index];
        let name = &current_country["name"]["common"];
        let capital = &current_country["capital"][0];
        print!("{} -> ",name);
        io::stdout().flush().unwrap();
        let input: String;
        scan!("{}",input);
        if input.as_str() == capital {
            println!("Correct");
            number_of_guessed_right += 1;
        } else {
            println!("Incorrect ({})",capital);
        }
    }
    println!("You guessed {}/{} right",number_of_guessed_right,n);
}

async fn body_to_string(req: Response<Body>) -> String {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    String::from_utf8(body_bytes.to_vec()).unwrap()
}