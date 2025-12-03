// Yes, it's kinda bad, but I am too lazy to download the input file manually
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    match std::env::var("ADVENT_OF_CODE_SESSION") {
        Ok(session) => {
            let text = client
            .get("https://adventofcode.com/2025/day/3/input")
            .header(reqwest::header::COOKIE, format!("session={}", session))
            .send()?
            .text()?;

            std::fs::write("data/input", text)?;
            println!("data/input is successfully written");

            Ok(())
        }

        Err(e) => {
            println!("ADVENT_OF_CODE_SESSION environment variable is not set");
            Err(Box::new(e))
        }
    }
}
