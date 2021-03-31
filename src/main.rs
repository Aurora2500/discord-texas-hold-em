use std::env;
use std::fs;
use std::time::Duration;
use std::sync::Arc;

use serenity::{
    async_trait,
    client::{
        Client, Context, EventHandler
    },
    collector::CollectReply,
    model::{
        user::User,
        channel::{
        Message, EmbedField
        }
    },
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup, StandardFramework,
        macros::{
            command, group
        }
    },
    utils::Colour
};

mod poker;

#[group]
#[commands(play_poker)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("a!")) // set the bot's prefix to "a!"
        .group(&GENERAL_GROUP);

    // Get token from file
    let token_filenname = &env::args().collect::<Vec<String>>()[1];
    let token = fs::read_to_string(token_filenname)
        .expect("Couldn't read file!");

    // Login with a bot token
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn reply(ctx: &Context, msg: &Message, user: &User) -> Option<User> {
    let message = msg.channel_id.await_reply(ctx)
        .author_id(user.id)
        .timeout(Duration::from_secs(20))
        .await;

    if let Some(ref msg) = message {
        if let Err(why) = msg.react(ctx, '✅').await {
            println!("REACT ERROR: {:?}", why);
        }
    }

    message.map(|msg| msg.author.clone())
}

fn format_list(l: &[User]) -> Option<String> {
    match l.len() {
        0 => None,
        1 => Some(format!("{}", l[0])),
        2 => Some(format!("{} and {}", l[0], l[1])),
        n => {
            let mut result = String::from("");
            for u in 0..(n-2) {
                result.push_str(&format!("{}, ", l[u]));
            }
            result.push_str(&format!("{} and {}", l[n-2], l[n-1]));
            Some(result)
        }
    }
}

#[command]
#[aliases("poker")]
async fn play_poker(ctx: &Context, msg: &Message) -> CommandResult {

    let tasks : Vec<_> =
        msg.mentions
        .iter()
        .map( |user| { reply(&ctx, &msg, user) })
        .collect();

    let players: Vec<User> =
        futures::future::join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<User>>();

    println!("PLAYERS: {:?}", players.iter().map(|u| &u.name).collect::<Vec<_>>());

    let mut names = format!("No one");
    if let Some(str) =  format_list(&players) {
        names = str;
    }
    msg.channel_id.say(ctx,format!("{} want(s) to play", names)).await;

    Ok(())
}

