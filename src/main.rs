use serde::{
    Serialize,
    Deserialize,
};
use rand::{
    self, 
    Rng,
};
use toml;
use std::fs;
use std::collections::HashMap;
use serenity::{
    self, 
    prelude::*, 
    model::channel::Message, 
    async_trait
};
mod cli;

#[derive(Serialize, Deserialize, Debug)]
struct User{
    name : String,
    money : i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config{
    token : String,
    intents : GatewayIntents,
    users : HashMap<String, User>,
}

impl Config{
    fn def() -> Config{
        return Config{
            token : "TOKEN".to_string(),
            intents : GatewayIntents::GUILD_MESSAGES|GatewayIntents::DIRECT_MESSAGES|GatewayIntents::MESSAGE_CONTENT,
            users : def_users(),
        };
    }
}

fn def_users() -> HashMap<String, User>{
        let mut usrs : HashMap<String, User> = HashMap::new();
        usrs.insert("Test".to_string(), User{name:"Test_name".to_string(), money : 0});
        return usrs;
}


#[tokio::main]
async fn main() {
    std::process::Command::new("clear").spawn().expect("Error");

    let configuration : Config = match get_config().await{
        Ok (v) => v,
        Err(e) => {
            println!("Error! {}", e);
            fs::write("configuration.toml", toml::to_string(&Config::def()).expect("Error converting Config to String"))
            .expect("Error writing to configuration.toml");
            println!("No configuration.toml detected! Generating custom config... \n CHANGE TOKEN BEFORE LAUNCHING AGAIN");
            return;
        }
    };

    if &configuration.token == "TOKEN"{
        println!("ERROR - NO TOKEN INPUTTED!");
    }

    let mut client = match Client::builder(&configuration.token, configuration.intents).event_handler(Handler).await{
        Ok (v) => v,
        Err (e) => {
            println!("Can't connect! {}", e);
            std::process::exit(0);
        }
    };

    cli::startup();

    if let Err(why) = client.start().await{
        println!("Error {why:?}");
    };
}

async fn get_config() -> Result<Config, Box<dyn std::error::Error>>{
    let conf_to_str = fs::read_to_string("configuration.toml")?;
    let to_return : Config =  toml::from_str(&conf_to_str)?;
    Ok(to_return)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx : Context, msg : Message){
        cli::formatted_msg(&msg.author.name, &msg.content);
        if &msg.content == "!bal"{
            msg.channel_id
                .say(&ctx, format!("*{}* posiada `{}`", msg.author.to_string(), get_balance(msg.author.name.to_string()).await))
                .await
                .expect("Error saying");
        }
        else if &msg.content == "!init"{

        }
        else if &msg.content == "!work"{
            let rand_num : i64 = rand::thread_rng()
                .gen_range(0..=10);
            add_money_to_user(rand_num, msg.author.name.to_string())
                .await
                .expect("error");
            msg.channel_id
                .say(&ctx, format!("Dodano `{}` na konto *{}*", rand_num, msg.author.to_string()))
                .await
                .expect("Error");
        }
    }
}

async fn add_user_properties(in_key : String, props : User) -> Result<(), Box<dyn std::error::Error>>{
    let mut database : Config = get_config().await?;
    database.users
        .insert(in_key, props);
    fs::write("configuration.toml", toml::to_string(&database).expect("Error parsing"))
        .expect("Error writing");
    Ok(())
}
async fn get_balance(in_key : String) -> i64{
    let database : Config = get_config()
        .await
        .expect("Error getting config");
    let to_return : Option<&User> = database.users.get(&in_key);

    if to_return.is_none(){
        add_user_properties(in_key.clone(), User{
            name : in_key.clone(),
            money : 0,
        }).await.expect("Err");
        return -0;
    }

    return to_return.unwrap().money;
}
async fn add_money_to_user(amount : i64, in_key : String) -> Result<(), Box<dyn std::error::Error>>{
    let mut database : Config = get_config()
        .await?;
    let usr_state : Option<&User> = Some(database.users.get(&in_key).expect("error"));

    if usr_state.is_none(){
        add_user_properties(in_key.clone(), User{
            name : in_key.clone(),
            money : 0
        }).await.expect("error");
    }

    database.users.insert(in_key, 
        User{
            name: usr_state.unwrap().name.clone(),
            money : usr_state.unwrap().money + amount,
        }
    );
    fs::write("configuration.toml", toml::to_string(&database).expect("Error parsing"))
        .expect("Error writing");
    Ok(())
}
