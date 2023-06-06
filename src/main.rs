use reqwest::Error;
use serde_derive::Deserialize;
use std::collections::HashMap;
use tokio::time::{interval, Duration};
use crossterm::terminal::{self};
use crossterm::ExecutableCommand;
use tokio::signal;
use futures::FutureExt;
use std::io::{self, Write, stdout};
use colored::*;
use textplots::{Chart, Plot, Shape};

#[derive(Deserialize, Debug)]
struct ProductSummary {
    sell_summary: Vec<BuySellSummary>,
    buy_summary: Vec<BuySellSummary>,
}

#[derive(Deserialize, Debug)]
struct BuySellSummary {
    // amount: u32,
    pricePerUnit: f64,
    // orders: u32,
}

#[derive(Deserialize, Debug)]
struct BazaarResponse {
    products: HashMap<String, ProductSummary>,
}

struct HistoricalData {
    buy_prices: Vec<(f32, f32)>,
    sell_prices: Vec<(f32, f32)>,
    time: f32,
    time_min: f32,
}

impl HistoricalData {
    fn new() -> Self {
        Self {
            buy_prices: vec![],
            sell_prices: vec![],
            time: 0.,
            time_min: 0.,
        }
    }

    fn add(&mut self, buy_price: f32, sell_price: f32) {
        self.buy_prices.push((self.time, buy_price));
        self.sell_prices.push((self.time, sell_price));
        self.time += 1.;

        if self.buy_prices.len() > 200 {
            self.buy_prices.remove(0);
            self.sell_prices.remove(0);
            self.time_min += 1.;
        }
    }
}

async fn fetch_and_print(data: &mut HistoricalData, product_id: &str) -> Result<bool, Error> {
    let response = reqwest::get("https://api.hypixel.net/skyblock/bazaar")
        .await?
        .json::<BazaarResponse>()
        .await?;

    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut product_found = false;

    for (product_key, product) in response.products {
        if product_key == product_id {
            product_found = true;
            let mut max_sell_price = 0.0;
            let mut min_sell_price = 99999999999.0;
            let mut max_buy_price = 0.0;
            let mut min_buy_price = 99999999999.0;

            println!("{}", format!("Product ID: {}", product_key).yellow());
            for summary in product.sell_summary {
                let current_amount:f64 = summary.pricePerUnit as f64;
                if min_sell_price > current_amount {
                    min_sell_price = current_amount;
                }
                if max_sell_price < current_amount {
                    max_sell_price = current_amount;
                }
            }
            for summary in product.buy_summary {
                let current_amount:f64 = summary.pricePerUnit as f64;
                if min_buy_price > current_amount {
                    min_buy_price = current_amount;
                }
                if max_buy_price < current_amount {
                    max_buy_price = current_amount;
                }
            }

            data.add(min_buy_price as f32, max_sell_price as f32);

            println!("Current Instant Buy Price: {}", format!(" {}", min_buy_price).green());
            println!("Current Instant Sell Price: {}", format!("{}", max_sell_price).red());
            println!("Current Buy Order Price: {}", format!("{}", max_sell_price).bright_green());
            println!("Current Sell Order Price: {}", format!("{}", min_buy_price).bright_red());

            println!("\nHistorical Buy Prices:");
            Chart::new(220, 50, data.time_min, data.time).lineplot(Shape::Lines(&data.buy_prices)).display();

            println!("\nHistorical Sell Prices:");
            Chart::new(220, 50, data.time_min, data.time).lineplot(Shape::Lines(&data.sell_prices)).display();
        }
    }

    // if !product_found {
    //     // eprintln!("No product with id {} found", product_id);
    // }

    Ok(product_found)
}

#[tokio::main]
async fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::EnterAlternateScreen).expect("Failed to enter alternate screen");

    let mut product_id;
    let mut product_found;
    
    loop {
        product_id = String::new();
        product_found = false;
        let mut retries = 0;
    
        print!("Enter the product id: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut product_id).unwrap();
    
        let original_product_id = product_id.clone();
    
        while !product_found {
            product_id = original_product_id.clone();
            product_id = product_id.trim_end().to_uppercase().replace(" ", "_").replace("-","_");
    
            if retries > 0 {
                if retries == 1 {
                    product_id = format!("{}_ITEM", product_id);
                } else if retries == 2 {
                    product_id = format!("ENCHANTMENT_ULTIMATE_{}", product_id);
                } else if retries == 3{
                    product_id = format!("ENCHANTMENT_{}", product_id)
                } else if retries == 4{
                    product_id = format!("{}_SCROLL", product_id)
                } else if retries == 5{
                    product_id = format!("{}_GEM", product_id)
                } else if retries == 6{
                    product_id = format!("{}_ORE", product_id)
                } else if retries == 7{
                    product_id = format!("DUNGEON_{}", product_id)
                }
                 else {
                    eprintln!("No product found with these modifications. Please try again with a new product id. If this item has a level (ex. enchanted book) make sure to include it");
                    break;
                }
            }
    
            let mut data = HistoricalData::new();
            match fetch_and_print(&mut data, &product_id).await {
                Ok(true) => product_found = true,
                Ok(false) => {
                    eprintln!("No product with id {} found. Trying with modifications.", original_product_id);
                    retries += 1;
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    
        if product_found {
            break;
        }
    }
    
    
    

    let mut interval = interval(Duration::from_secs(10));
    let mut data = HistoricalData::new();

    let ctrl_c = signal::ctrl_c().fuse();

    let main_loop = async {
        loop {
            interval.tick().await;
            if let Err(e) = fetch_and_print(&mut data, &product_id).await {
                eprintln!("Error: {}", e);
            }
        }
    }.fuse();

    tokio::pin!(ctrl_c, main_loop);

    tokio::select! {
        _ = &mut ctrl_c => {
            println!("Ctrl-C received... goodbye");
        },
        _ = &mut main_loop => { /* this will never be reached */ },
    }

    stdout.execute(terminal::LeaveAlternateScreen).expect("Failed to leave alternate screen");
}

