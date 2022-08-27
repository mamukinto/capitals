use std::{io, fmt::format};
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
    let data = json::parse(&response_content_json).unwrap();
    println!("Ready to go");
    println!("");
    println!("");
    println!("---------------------------------------------------------------");
    play(&data);
    
    Ok(())
}

fn play(data: &JsonValue) {
    print!("Enter number of countries to guess: ");
    io::stdout().flush().unwrap();
    let choice : i32;
    scan!("{}",choice);
    play_guess(choice, &data);
}
fn guess(data: &JsonValue) -> (bool,String) {
    let current_country_index = thread_rng().gen_range(0..249);
        
    let current_country = &data[current_country_index];
    let name = &current_country["name"]["common"];
    let capital = &current_country["capital"][0];
    print!("{} -> ",name);
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    scan!("{}",input);
    let guess_percentage = get_guess_pecentage(&input, &capital.to_string());

    match guess_percentage {
        100 => {
            (true,format!("Correct!"))
        },
        66..=99 => {
            (false,format!("Almost got it right! [{}%] ({})",guess_percentage,capital))
        }
        33..=65 => {
            (false,format!("Could have done better! [{}%] ({})",guess_percentage,capital))
        }
        0..=32 => {
            (false,format!("Not close! [{}%] ({})",guess_percentage,capital))
        }
        _ => {
            (false,format!("Unexpected matching percentage {}", guess_percentage))
        }
    }

}
fn play_guess(number_of_total_guesses : i32,data: &JsonValue) {
    let mut number_of_guessed_right = 0;
    for _ in 0..number_of_total_guesses {
        let result = guess(&data);
        if result.0 {
            number_of_guessed_right += 1;
        } 
        println!("{}",result.1)
    }
    println!("You guessed {}/{} right, want to play again? (y/n)",number_of_guessed_right,number_of_total_guesses);
    let choice : String;
    scan!("{}",choice);
    match choice.as_str() {
        "y" => play(&data),
        "n" => print!("Thank you for playing, Hope to see you again!"),
        _ => {}
    };
}

async fn body_to_string(req: Response<Body>) -> String {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    String::from_utf8(body_bytes.to_vec()).unwrap()
}
fn lower(a : &String,b : &String) -> usize {
    if a.len() < b.len() {
        a.len()
    } else {
        b.len()
    }
}
fn higher(a : &String,b : &String) -> usize {
    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}

fn get_guess_pecentage(guess: &String, capital: &String) -> usize {
    let mut count_of_matched_chars = 0;
    let min_len = lower(guess,capital);
    for i in 0..min_len {
        if guess.chars().nth(i) == capital.chars().nth(i) {
            count_of_matched_chars += 1;
        }
    }

    (100 * count_of_matched_chars) / higher(&guess, &capital)

} 