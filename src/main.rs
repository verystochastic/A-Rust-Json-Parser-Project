// src/main.rs
use quadrivium::NasaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quadrivium - Lunar Eclipse Tracker\n");
    
    let nasa_client = NasaClient::new();
    
    println!("Fetching Moon position data...");
    match nasa_client.get_moon_position().await {
        Ok(moon_data) => {
            println!("Moon position data:");
            println!("{}", moon_data);
        }
        Err(e) => {
            println!("Error fetching moon data: {}", e);
        }
    }
    
    println!("\nFetching Sun position data...");
    match nasa_client.get_sun_position().await {
        Ok(sun_data) => {
            println!("Sun position data:");
            println!("{}", sun_data);
        }
        Err(e) => {
            println!("Error fetching sun data: {}", e);
        }
    }
    
    Ok(())
}
