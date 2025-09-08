// src/data/nasa.rs
use crate::core::json::{JsonValue, Parser};

pub struct NasaClient {
    horizons_url: String,
}

impl NasaClient {
    pub fn new() -> Self {
        Self {
            horizons_url: "https://ssd.jpl.nasa.gov/api/horizons.api".to_string(),
        }
    }

    pub async fn get_moon_position(&self) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Moon = 301, observed from Earth center (399)
        let url = format!(
            "{}?format=json&COMMAND='301'&OBJ_DATA='YES'&MAKE_EPHEM='YES'&EPHEM_TYPE='OBSERVER'&CENTER='399'&START_TIME='2024-09-07'&STOP_TIME='2024-09-08'&STEP_SIZE='1%20h'&QUANTITIES='1,20'", 
            self.horizons_url
        );
        
        println!("Requesting Moon data from: {}", url);
        
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        
        println!("Response (first 500 chars): {}", &text.chars().take(500).collect::<String>());
        
        let mut parser = Parser::new(&text);
        let json_value = parser.parse()?;
        
        Ok(json_value)
    }

    pub async fn get_sun_position(&self) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Sun = 10, observed from Earth center (399)  
        let url = format!(
            "{}?format=json&COMMAND='10'&OBJ_DATA='YES'&MAKE_EPHEM='YES'&EPHEM_TYPE='OBSERVER'&CENTER='399'&START_TIME='2024-09-07'&STOP_TIME='2024-09-08'&STEP_SIZE='1%20h'&QUANTITIES='1,20'",
            self.horizons_url
        );
        
        println!("Requesting Sun data from: {}", url);
        
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        
        println!("Response (first 500 chars): {}", &text.chars().take(500).collect::<String>());
        
        let mut parser = Parser::new(&text);
        let json_value = parser.parse()?;
        
        Ok(json_value)
    }
}
