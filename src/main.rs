mod api;
mod meow;

slint::include_modules!();

const TARGET: &[(&str, &str)] = &[
    ("bitcoin", "BTC"),
    ("ethereum", "ETH"),
    ("tether", "USDT"),
    ("tron", "TRX"),
    ("binancecoin", "BNB"),
    ("shiba-inu", "SHIB"),
    ("dogecoin", "DOGE"),
    ("ripple", "XRP"),
    ("cardano", "ADA"),
    ("polkadot", "DOT"),
    ("solana", "SOL"),
    ("matic-network", "MATIC"),
    ("ethereum-classic", "ETC"),
    ("litecoin", "LTC"),
    ("stellar", "XLM"),
    ("cosmos", "ATOM"),
    ("hedera", "HBAR"),
    ("avalanche-2", "AVAX"),
    ("ftx-token", "FTT"),
    ("aave", "AAVE"),
];

#[tokio::main]
async fn main() {
    let ui = AppWindow::new().unwrap();


    {
        let ui_handle = ui.clone_strong();
        ui.on_request_update(move || {
            let ui = ui_handle.clone_strong();
            slint::spawn_local(async move {
                update_coins(&ui).await;
            }).unwrap();
        });
    }

    {
        let ui_clone = ui.clone_strong();
        slint::spawn_local(async move {
            // wait a bit for the app to fully initialize
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            update_coins(&ui_clone).await;
        }).unwrap();
    }

    ui.run().unwrap();
}

async fn update_coins(ui: &AppWindow) {
    match api::fetch_prices().await {
        Ok(data) => {
            println!("Successfully fetched prices, map size: {}", data.prices.len());
            
            let coins: Vec<Coin> = TARGET.iter()
                .filter_map(|(api_key, symbol)| {
                    data.prices.get(*api_key).map(|p| {
                        println!("Found coin: {} ({}) = {} Toman", symbol, api_key, p);
                        Coin {
                            symbol: (*symbol).into(),
                            price: format_toman_price(*p),
                        }
                    })
                })
                .collect();

            println!("Total coins found: {}", coins.len());
            
            let slint_data = slint::VecModel::from(coins);
            ui.set_coins(slint::ModelRc::new(slint_data));
            let exchange_text = format!(
                "1 دلار = {} تومان",
                format_with_commas(data.exchange_rate.round() as i64)
            );
            ui.set_exchange_rate(exchange_text.into());
        }
        Err(e) => {
            eprintln!("Error fetching prices: {}", e);
            ui.set_coins(slint::ModelRc::new(slint::VecModel::from(vec![])));
            ui.set_exchange_rate("نتوانست نرخ تبدیل را دریافت کند".into());
        }
    }
}

fn format_toman_price(price: f64) -> slint::SharedString {
    let n = price.round() as i64;
    format!("{} تومان", format_with_commas(n)).into()
}

fn format_with_commas(n: i64) -> String {
    let mut s = n.abs().to_string();
    let mut out = String::new();
    let mut count = 0;
    for ch in s.chars().rev() {
        if count != 0 && count % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
        count += 1;
    }
    if n < 0 {
        out.push('-');
    }
    out.chars().rev().collect()
}
