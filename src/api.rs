use reqwest;
use std::collections::HashMap;
use std::time::Duration;

pub struct PriceData {
    pub prices: HashMap<String, f64>,
    pub exchange_rate: f64,
}

pub async fn fetch_prices() -> Result<PriceData, Box<dyn std::error::Error>> {
    let ids = "bitcoin,ethereum,tether,tron,binancecoin,shiba-inu,dogecoin,ripple,cardano,polkadot,solana,matic-network,ethereum-classic,litecoin,stellar,cosmos,hedera,avalanche-2,ftx-token,aave";
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", ids);
    
    println!("Fetching prices from CoinGecko API...");
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let response = client.get(&url).send().await?;
    
    println!("Response status: {}", response.status());
    
    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()).into());
    }
    
    let text = response.text().await?;
    println!("Response received, parsing...");
    
    let json: serde_json::Value = serde_json::from_str(&text)?;
        println!("Fetching USD to Toman exchange rate...");
    let exchange_rate = fetch_usd_to_toman_rate().await.unwrap_or(140000.0);
    println!("Exchange rate: 1 USD = {} Toman", exchange_rate);
    
    // پارسینگ JSON بر اساس ساختار CoinGecko
    let mut prices = HashMap::new();
    
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if let Some(price_obj) = value.as_object() {
                if let Some(price_num) = price_obj.get("usd").and_then(|v| v.as_f64()) {
                    // تبدیل به تومن
                    let price_in_toman = price_num * exchange_rate;
                    prices.insert(key.clone(), price_in_toman);
                }
            }
        }
    }
    
    println!("Successfully parsed {} prices", prices.len());
    
    Ok(PriceData {
        prices,
        exchange_rate,
    })
}

async fn fetch_usd_to_toman_rate() -> Result<f64, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    println!("Trying first exchange rate API...");
    if let Ok(response) = client
        .get("https://api.exchangerate-api.com/v4/latest/usd")
        .send()
        .await
    {
        println!("First API response status: {}", response.status());
        if let Ok(text) = response.text().await {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(rate) = json
                    .get("rates")
                    .and_then(|r| r.get("IRR"))
                    .and_then(|v| v.as_f64())
                {
                    let realistic_rate = rate * 3.3; // تقریبی برای نزدیک به نرخ واقعی بازار
                    println!("Official rate: {}, realistic market rate: {}", rate, realistic_rate);
                    return Ok(realistic_rate);
                }
            }
        }
    }
    println!("Trying second exchange rate API...");
    if let Ok(response) = client
        .get("https://v6.exchangerate-api.com/v6/latest/USD")
        .send()
        .await
    {
        println!("Second API response status: {}", response.status());
        if let Ok(text) = response.text().await {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(rate) = json
                    .get("conversion_rates")
                    .and_then(|r| r.get("IRR"))
                    .and_then(|v| v.as_f64())
                {
                    println!("Successfully fetched exchange rate from API 2: {}", rate);
                    return Ok(rate);
                }
            }
        }
    }
    
    println!("Both exchange rate APIs failed, using fallback rate");
    Err("Could not fetch exchange rate, using default".into())
}
