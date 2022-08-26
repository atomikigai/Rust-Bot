use reqwest::header::{USER_AGENT, CONTENT_TYPE, ACCEPT};
use std::{error::Error};
use crate::crypto_currencies::coin_struct::{Coin, Deserialize, Value};

#[derive(Deserialize)]
struct Quote{
    price: String,
}

#[derive(Deserialize)]
struct Volume{
    volume_change_24h: String
}

#[derive(Deserialize)]
pub struct  CoinData{
    id: String,
    name: String,
    symbol: String,
    quote: Quote,
    volume_change_24h: Volume
}


impl std::fmt::Display for CoinData{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nID: {} 🔑 \nName: {} 🎲 \nSymbol: {} 🚀 \nPrice: {} \nVolume24h: {} ",
         self.id, self.name, self.symbol, self.quote, self.volume_change_24h)
    }
} 

impl std::fmt::Display for Quote{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} USD 💸", self.price)
    }
}

impl std::fmt::Display for Volume{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num:f64 = 0.0;
        if self.volume_change_24h.is_empty(){
            num = 0.0;
        }else{
            num = self.volume_change_24h.parse().unwrap();
        }
        let is_positive = if num < 0.0 {
            false
        }else{
            true
        };

        match is_positive{
            true => write!(f, "+{} 📈​📊​​", self.volume_change_24h),
            false => write!(f, "{} ​​📉​📊​​", self.volume_change_24h)
        }
    }
}

impl CoinData{

    pub fn new() -> Self{
        CoinData{
            id: String::new(),
            name: String::new(),
            symbol: String::new(),
            quote: Quote { price: String::new() },
            volume_change_24h: Volume { volume_change_24h: String::new() }
        }
    }

    fn build(x: &Value) -> Self{
        CoinData{
            id: CoinData::check_type(&x["id"]),
            name: CoinData::check_type(&x["name"]),
            symbol: CoinData::check_type(&x["symbol"]),
            quote: Quote { price: CoinData::check_type(&x["quote"]["USD"]["price"]) },
            volume_change_24h: Volume { volume_change_24h: CoinData::check_type(&x["quote"]["USD"]["volume_change_24h"]) } 
        }
    }

    pub async fn get_coins() -> Result<String, Box<dyn Error + Send + Sync>>{
        let client = reqwest::Client::new();
        let _res =  client.get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?limit=100")
        .header(USER_AGENT, "Rust Bot")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("X-CMC_PRO_API_KEY", "800ab4c9-8cca-4cdc-88a0-0aa06247fe60")
        .send()
        .await?
        .text()
        .await?;
        
        Ok(_res)
    }
    

    pub fn find_coin(map: Coin, coin_name: String) -> CoinData{
        let mut result = CoinData::new();
        for x in &map.data{
            let actual_name = CoinData::check_type(&x["name"]).to_lowercase();
            let actual_symbol = CoinData::check_type(&x["symbol"]).to_lowercase();
            if coin_name ==  actual_name || coin_name == actual_symbol{
                result = CoinData::build(x);
                break;
            }else{
                result = CoinData::new();
            }
        }
        result
    } 
    
    fn check_type(word: &Value) -> String{
        if word.is_number(){
           
           let number = word.to_string();
           let decimal_len = CoinData::count_decimals(&number);
           
           if decimal_len > 1 {
            let new_number_format = CoinData::convert_to_decimal(&number);
            let num = format!("{:.1$}", new_number_format, 2);
            num
           }else{
            let new_number_format = CoinData::convert_to_int(&number);
            let num = format!("{:.1$}", new_number_format, 1);
            num
           }
           
        }else{
            if word.is_string(){
               let st = CoinData::remove_quotes(word);
                st
            }else{
                if word.is_boolean() {
                    let option = String::from("Si");
                    option
                }else{
                    let option = String::from("No");
                    option
                }
            }
        }
    }

    fn count_decimals(word: &String) -> u8{
        let mut found = false;
        let mut decimal_count: u8 = 0;
        for (_x,y) in word.chars().enumerate(){
            match y {
                '.' => found = true,
                _=> ()
            }

            if found {
                decimal_count += 1;
            }
        }

        decimal_count
    }

    fn convert_to_decimal(number: &String) -> f64{
        number.parse::<f64>().expect("Expect a number")
    }

    fn convert_to_int(number: &String) -> i32{
        number.parse::<i32>().expect("Expect a number")
    }

    fn remove_quotes(word: &Value) -> String{
        // otc: 042 hex: 22 dec: 34 = ""
        let word = word.to_string();
        let len = &word.len();
        let final_s = String::from(&word[1..len-1]);
        final_s
    }
}
