use std::env;
use teloxide::{prelude::*, dispatching::update_listeners::webhooks, utils::command::{BotCommands}};
use std::{error::Error};
mod crypto_currencies;
use crypto_currencies::{coin_data::CoinData};
use crypto_currencies::coin_struct::Coin;
use url::Url;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Bot");
    let bot = Bot::from_env().auto_send();
    let token = bot.inner().token();

    let port: u16 = env::var("PORT")
    .expect("PORT env variable is not set")
    .parse()
    .expect("PORT env variable value is not an integer");

    let addr = ([0,0,0,0], port).into();
    let host = env::var("HOST").expect("HOST variable is not set");
    let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
    
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
    .await
    .expect("Couldn't setup webhook");

    teloxide::commands_repl_with_listener(
        bot,
        answer,
        listener,
        Command::ty(),
    )
    .await;

}

#[derive(BotCommands, Clone)]
#[command(rename="lowercase", description="Estos son los comandos: ")]
enum Command{
    #[command(description = "Muestra todos los comandos.")]
    Help,
    #[command(description = "Recibe tu nombre")]
    Username(String),
    #[command(description = "Consulta estadisticas de una criptomoneda /GetPrice btc")]
    Getprice(String),

}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>>{
    match command{
        Command::Help =>{
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        },
        Command::Username(username) =>{
            bot.send_message(message.chat.id, format!("Tu usuario es @{username}.")).await?
        },
        Command::Getprice(coin_name) =>{
            let data = CoinData::get_coins().await.expect("Expect Data");
            let s: Coin  = serde_json::from_str(&data[..]).unwrap();
            let new_coin = CoinData::find_coin(s, coin_name.to_lowercase());
            bot.send_message(message.chat.id, format!("​​📊 ​Data​ 📊  📌 {}", new_coin)).await?
        } 
    };
    Ok(())
}



