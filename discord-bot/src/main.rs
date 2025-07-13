use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use subslay::EmojiStylist;
use dotenv::dotenv;
use std::env;

struct Handler {
    stylist: EmojiStylist,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content.starts_with("!slay ") {
            let input = msg.content.trim_start_matches("!slay ");
            let output = self.stylist.slay(input).join(" ");

            if let Err(why) = msg.channel_id.say(&ctx.http, output).await {
                eprintln!("❌ Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ SubSlay Bot is online as {}!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("❌ DISCORD_TOKEN must be set in .env");

    let stylist = EmojiStylist::new().expect("❌ Failed to initialize EmojiStylist");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler { stylist };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("❌ Error creating Discord client");

    if let Err(e) = client.start().await {
        eprintln!("❌ Client error: {:?}", e);
    }
}
