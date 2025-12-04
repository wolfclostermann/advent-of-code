use reqwest;
use substring::Substring;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://adventofcode.com/2025/day/1/input";
    let session_token = "session=[snip]";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Cookie", session_token)
        .send()
        .await?;
    let body = res.text().await?;

    let start = 50;
    let mut pos = start;
    let mut password: i32 = 0;
    let mut i = 0;

    for movement in body.lines() {
        i = i + 1;
        let direction = movement.substring(0, 1);
        let mut movement_count: i32 = movement.substring(1, movement.chars().count()).parse()?;
        password = password + (movement_count / 100);
        movement_count = movement_count % 100;
        match direction {
            "L" => {
                let new_pos = pos - movement_count;
                let corrected_pos = match new_pos {
                    x if x < 0 => {
                        if pos > 0 {
                            println!("Went through zero, added 1 to the password");
                            password = password + 1;
                        };
                        100 + new_pos
                    },
                    x if x >= 0 => new_pos,
                    _ => panic!()
                };
                pos = corrected_pos;
            },
            "R" => {
                let new_pos = pos + movement_count;
                let corrected_pos = match new_pos {
                    x if x <= 99 => new_pos,
                    x if x > 100 => {
                        println!("Went through zero, added 1 to the password");
                        password = password +1;
                        new_pos - 100
                    },
                    100 => 0,
                    _ => panic!()
                };
                pos = corrected_pos;                    
            },
            _ => panic!()
        };
        match pos {
            0 => password = password + 1,
            _ => ()
        };
        println!("{}. Moved {} {}, ended up at {}", i, direction, movement_count, pos);
    }
    println!("Final pos {} password {}", pos, password);
    Ok(())
}
