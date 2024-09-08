use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::read_to_string;
use std::fs::File;
use std::process::{Command, Output};
use serenity::async_trait;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use rand::Rng;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == String::from("TimeParadox_Bot") {
            return;
        }
        let msg_content: String = msg.content.clone();
        println!("CONTENT: |{msg_content}|");
        let mut message: Vec<String> = msg_content.replacen('\n', " ", 1).split(" ").map(|x| x.to_string()).collect();
        let command: String = message.remove(0);
        let args = message.join(" ");
        println!("ARGS: |{args}|");
        match command.as_str() {
            "!ping" => { let _ = msg.channel_id.say(&ctx.http, "pong!").await; }
            "!render" => {
                let mut file = File::create("buf.typ").unwrap();
                let mut content: String = String::from("#set text(fill: rgb(\"eeeeee\"))\n#set page(fill: rgb(\"313338\"), width: auto, height: auto, margin: (top: 0.3cm, bottom: 0.3cm, left: 0.3cm, right: 0.3cm))\n");
                let args = args.split("```").map(|x| x.to_string()).collect::<Vec<String>>();
                content += &args[1];
                let mut ppi: usize = 256;
                if content.len() > 1 {
                    let ppi_arg: String = args[2].clone();
                    let split_arg: Vec<String> = ppi_arg.split("=").map(|x| x.to_string()).collect();
                    if split_arg[0] == String::from(" --ppi") {
                        if let Ok(ppi_val) = &split_arg[1].parse() {
                            ppi = *ppi_val;
                        } else {
                            let _ = msg.channel_id.say(&ctx.http, "Invalid PPI argument. Expected integer.");
                        }
                    }
                }
                let _ = file.write_all(content.as_bytes());
                match Command::new("typst").arg("compile").arg("--ppi").arg(ppi.to_string()).arg("buf.typ").arg("buf.png").output() {
                    Ok(o) if o.status.success() => {
                        let attachment = CreateAttachment::path(Path::new("buf.png")).await.unwrap();
                        let _ = msg.channel_id.send_files(&ctx, vec![attachment], CreateMessage::new().content("")).await.unwrap();
                    }
                    Ok(Output { stderr, .. }) => {
                        let error_message = String::from_utf8_lossy(&stderr);
                        let _ = msg.channel_id.say(&ctx.http, format!("Typst compile error: {}", error_message)).await;
                    }
                    Err(e) => {
                        let _ = msg.channel_id.say(&ctx.http, &format!("Typst compile error.\n{e}"));
                    }
                }
            }
            "!rm" => {
                let mut file = File::create("buf.typ").unwrap();
                let mut content: String = String::from("#set text(fill: rgb(\"eeeeee\"))\n#set page(fill: rgb(\"313338\"), width: auto, height: auto, margin: (top: 0.3cm, bottom: 0.3cm, left: 0.3cm, right: 0.3cm))\n");
                let args = args.split("```").map(|x| x.to_string()).collect::<Vec<String>>();
                content += "$";
                content += &args[1];
                content += "$";
                let mut ppi: usize = 256;
                if content.len() > 1 {
                    let ppi_arg: String = args[2].clone();
                    let split_arg: Vec<String> = ppi_arg.split("=").map(|x| x.to_string()).collect();
                    if split_arg[0] == String::from(" --ppi") {
                        if let Ok(ppi_val) = &split_arg[1].parse() {
                            ppi = *ppi_val;
                        } else {
                            let _ = msg.channel_id.say(&ctx.http, "Invalid PPI argument. Expected integer.");
                        }
                    }
                }
                let _ = file.write_all(content.as_bytes());
                match Command::new("typst").arg("compile").arg("--ppi").arg(ppi.to_string()).arg("buf.typ").arg("buf.png").output() {
                    Ok(o) if o.status.success() => {
                        let attachment = CreateAttachment::path(Path::new("buf.png")).await.unwrap();
                        let _ = msg.channel_id.send_files(&ctx, vec![attachment], CreateMessage::new().content("")).await.unwrap();
                    }
                    Ok(Output { stderr, .. }) => {
                        let error_message = String::from_utf8_lossy(&stderr);
                        let _ = msg.channel_id.say(&ctx.http, format!("Typst compile error: {}", error_message)).await;
                    }
                    Err(e) => {
                        let _ = msg.channel_id.say(&ctx.http, &format!("Typst compile error.\n{e}"));
                    }
                }
            }
            "!qm" => {
                let mut file = File::create("buf.typ").unwrap();
                let mut content: String = String::from("#set text(fill: rgb(\"eeeeee\"))\n#set page(fill: rgb(\"313338\"), width: auto, height: auto, margin: (top: 0.3cm, bottom: 0.3cm, left: 0.3cm, right: 0.3cm))\n");
                content += "$";
                content += &args;
                content += "$";
                let mut ppi: usize = 256;
                if content.len() > 1 {
                    let ppi_arg: String = args.split(' ').map(|x| x.to_string()).collect::<Vec<String>>().last().unwrap().to_string();
                    let split_arg: Vec<String> = ppi_arg.split("=").map(|x| x.to_string()).collect();
                    if split_arg[0] == String::from(" --ppi") {
                        if let Ok(ppi_val) = &split_arg[1].parse() {
                            ppi = *ppi_val;
                        } else {
                            let _ = msg.channel_id.say(&ctx.http, "Invalid PPI argument. Expected integer.");
                        }
                    }
                }
                let _ = file.write_all(content.as_bytes());
                match Command::new("typst").arg("compile").arg("--ppi").arg(ppi.to_string()).arg("buf.typ").arg("buf.png").output() {
                    Ok(o) if o.status.success() => {
                        let attachment = CreateAttachment::path(Path::new("buf.png")).await.unwrap();
                        let _ = msg.channel_id.send_files(&ctx, vec![attachment], CreateMessage::new().content("")).await.unwrap();
                    }
                    Ok(Output { stderr, .. }) => {
                        let error_message = String::from_utf8_lossy(&stderr);
                        let _ = msg.channel_id.say(&ctx.http, format!("Typst compile error: {}", error_message)).await;
                    }
                    Err(e) => {
                        let _ = msg.channel_id.say(&ctx.http, &format!("Typst compile error.\n{e}"));
                    }
                }
            }
            "!affirmation" => {
                let raw: String = read_to_string("affirmations.txt").unwrap();
                let contents: Vec<String> = raw.split('\n').map(|x| x.to_string()).collect();
                let idx: usize = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..contents.len())
                };
                let message: String = contents[idx].clone();
                let _ = msg.channel_id.say(&ctx.http, message).await;
            }
            "!anatomy" => {
                let raw: String = read_to_string("anatomy.txt").unwrap();
                let contents: Vec<String> = raw.split('\n').map(|x| x.to_string()).collect();
                let idx: usize = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..contents.len())
                };
                let message: String = contents[idx].clone();
                let _ = msg.channel_id.say(&ctx.http, message).await;
            }
            "!fact" => {
                let raw: String = read_to_string("facts.txt").unwrap();
                let contents: Vec<String> = raw.split('\n').map(|x| x.to_string()).collect();
                let idx: usize = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..contents.len())
                };
                let message: String = contents[idx].clone();
                let _ = msg.channel_id.say(&ctx.http, message).await;
            }
            _ => {
                if let Some(c) = command.chars().nth(0) {
                    if c == '!' { let _ = msg.channel_id.say(&ctx.http, "Invalid command.").await; }
                    else {
                        let dollar_indices: Vec<usize> = args.match_indices('$').map(|x| x.0).collect();
                        if dollar_indices.len() == 2 {
                            let start: usize = dollar_indices[0] + 1;
                            let end: usize = dollar_indices[1];
                            let result = &args[start..end];
                            let mut file = File::create("buf.typ").unwrap();
                            let mut content: String = String::from("#set text(fill: rgb(\"eeeeee\"))\n#set page(fill: rgb(\"313338\"), width: auto, height: auto, margin: (top: 0.3cm, bottom: 0.3cm, left: 0.3cm, right: 0.3cm))\n");
                            content += "$";
                            content += result;
                            content += "$";
                            let _ = file.write_all(content.as_bytes());
                            match Command::new("typst").arg("compile").arg("buf.typ").arg("buf.png").output() {
                                Ok(o) if o.status.success() => {
                                    let attachment = CreateAttachment::path(Path::new("buf.png")).await.unwrap();
                                    let _ = msg.channel_id.send_files(&ctx, vec![attachment], CreateMessage::new().content("")).await.unwrap();
                                }
                                Ok(Output { stderr, .. }) => {
                                    let error_message = String::from_utf8_lossy(&stderr);
                                    let _ = msg.channel_id.say(&ctx.http, format!("Typst compile error: {}", error_message)).await;
                                }
                                Err(e) => {
                                    let _ = msg.channel_id.say(&ctx.http, &format!("Typst compile error.\n{e}"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("Store bot token in environment variable.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    match client.start().await {
        Ok(_) => {}
        Err(e) => { eprintln!("Error starting bot: {e}."); }
    }
}
