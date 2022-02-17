#![allow(dead_code)]
#![allow(unused_variables)]

mod translate;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::prelude::{GuildStatus, GuildUnavailable, GuildId};
use serenity::model::event::{TypingStartEvent};
use serenity::model::channel::{ReactionType, Channel};
use serenity::model::id::EmojiId;
use std::borrow::Borrow;
use std::fs::File;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use serenity::model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteractionDataOptionValue};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::model::prelude::application_command::ApplicationCommandOptionType;

struct Handler;
static MAIN_GUILD_ID: u64 = 745725474465906732;
static MAIN_GUILD: GuildStatus = GuildStatus::Offline(GuildUnavailable { id: GuildId{ 0: MAIN_GUILD_ID }, unavailable: true });
static HAHAYES_EMOTE: u64 = 627151632534339595;
static MOG_EMOTE: u64 = 745729242423099585;
static THEFLIP_EMOTE: u64 = 758463821831471174;
static IRCH_UID: u64 = 292362225388355584;
static MARK_UID: u64 = 179024507657256960;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Word {
    word: String,
    count: u32,
}

fn rec2word(mut rdr: Reader<File>) -> Vec<Word> {
    let mut v = vec![];
    for result in rdr.records() {
        if result.is_err() {
            continue;
        }
        let rec = result.unwrap();
        if rec.len() != 2 {
            continue;
        }
        let w: Result<Word, csv::Error> = rec.deserialize(None);
        if w.is_err() {
            continue
        }
        v.push(w.unwrap());
    }
    v.clone()
}

fn words2counts(words: Vec<String>) -> Vec<Word> {
    let mut v: Vec<Word> = vec![];
    for word in words {
        if word == "" {
            continue
        }
        let mut contains = false;
        let mut v_index = 0;
        for mut w in v.clone() {
            if w.word == word {
                v.remove(v_index);
                w.count += 1;
                v.insert(v_index, w);
                contains = true;
            }
            v_index += 1;
        }
        if !contains {
            v.push(Word{ word, count: 1 })
        }
    }
    v
}

fn counts2fields(counts: Vec<Word>, num: u32) -> Vec<(String, String, bool)> {
    let mut fields = vec![];
    let mut count = 0;
    for word in counts {
        let field1 = "Nr. ".to_owned() + (count+1).to_string().as_str();
        fields.push((word.clone().word, word.count.to_string(), true));
        count += 1;
        if count >= num {
            break;
        }
    }
    fields
}

#[async_trait]
impl EventHandler for Handler {
    // Gets called every time someone sends a message in channel this bot can see.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return; }
        if msg.author.id == MARK_UID {
            msg.react(ctx.borrow(), ReactionType::Custom {
                animated: true,
                id: EmojiId(THEFLIP_EMOTE),
                name: Option::Some(String::from("TheFlip"))
            }).await;
        }

        println!("MSG from {:?}: {:?}", msg.author.name, msg.content);

        let mut msg_words = vec![];
        for word in msg.content.split(&[' ', '\n', '.', ',', '?', '!'][..]) {
            msg_words.push(word.to_lowercase());
        }

        let mut rdr = csv::Reader::from_path("./word_count.csv");
        if rdr.is_ok() {
            // SETUP
            let mut rdr = rdr.unwrap();
            let mut old_counts = rec2word(rdr);
            let new_counts = words2counts(msg_words);
            let mut wtr = Writer::from_path("word_count.csv");
            // pog: 1

            // LOOP AND WRITE ALL WORDS AGAIN
            for new_word in new_counts {
                let mut found = false;
                let mut old_counts_index = 0;
                for mut old_word in old_counts.clone() {
                    // new_word: "pog", count: 1
                    if new_word.word == old_word.word {
                        old_counts.remove(old_counts_index);
                        old_word.count += new_word.count;
                        old_counts.insert(old_counts_index, old_word);
                        found = true;
                        break;
                    }
                    old_counts_index += 1;
                }
                if !found {
                    old_counts.push(Word{ word: new_word.word.to_string(), count: new_word.count })
                }
            }
            if wtr.is_ok() {
                let mut wtr = wtr.unwrap();
                wtr.write_record(&["word", "count"]);
                //wtr.write_record(&[w.word.clone(), w.count.to_string()]);

                for word in old_counts {
                    wtr.write_record(&[word.word.clone(), word.count.to_string()]);
                }
                wtr.flush();
            }
        } else {
            println!("rdr error: {}", rdr.err().unwrap());
        }
        /*if let Err(why) = msg.channel_id.say(&ctx.http, msg.content).await {
            println!("Error sending message: {:?}", why);
        }*/
    }

    // Called once at startup.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}!", ready.user.name);
        for g in ready.guilds {
            if g.id().0 == MAIN_GUILD_ID {
                if g.id() == MAIN_GUILD.id() {
                    println!("GUILD: {:?}", g);
                    println!("GUILD: {:?}", MAIN_GUILD);
                }
            }
        }

        // Create commands
        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("test").description("test desc")
                })
                .create_application_command(|command| {
                    command.name("cool").description("It is a cool command!")
                })
                .create_application_command(|command| {
                    command.name("topwords").description("Displays top words said")
                })
                .create_application_command(|command| {
                    command.name("wordcount").description("Displays top words said").create_option(|o| {
                        o.name("word")
                            .description("The word to look up")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                })
        }).await;

        let guild_command = MAIN_GUILD.id()
            .create_application_command(&ctx.http, |command| {
                command.name("guild_test").description("test that only works in one guild")
            }).await;
    }

    // Called whenever a user starts typing
    async fn typing_start(&self, ctx: Context, typing_event: TypingStartEvent) {
        let user = typing_event.user_id.to_user(ctx.borrow()).await;
        let channel = typing_event.channel_id.to_channel(ctx).await;

        if user.is_ok() {
            print!("User {:?} started typing ", user.unwrap().name);
        } else {
            print!("User {:?} started typing ", user);
        }

        if channel.is_ok() {
            println!("in: {}",
                match channel.unwrap() {
                    Channel::Guild(ch) => { ch.name }
                    Channel::Private(ch) => { ch.name() }
                    Channel::Category(ch) => { ch.name }
                    _ => {String::from("Unknown")}
                }
            );
        } else {
            println!("in: {:?}", channel);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Got interaction");
        if let Interaction::ApplicationCommand(command) = interaction {
            let mut embed = false;
            let mut title = "";
            let mut fields = vec![];
            let content = match command.data.name.as_str() {
                "test" => translate::test_translate("Hello Mr. Pog!").await,
                "cool" => "YOUR MOM LOL".to_string(),
                "topwords" => {
                    embed = true;
                    title = "Top 10 words used so far:";

                    let mut rdr = csv::Reader::from_path("./word_count.csv");
                    if rdr.is_ok() {
                        let mut rdr = rdr.unwrap();
                        let mut old_counts = rec2word(rdr);
                        old_counts.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap());
                        fields = counts2fields(old_counts, 10);
                        let mut result = "```\n".to_string();
                        for field in fields {
                            result += field.0.as_str();
                            result += ": ";
                            result += field.1.as_str();
                            result += "\n";
                        }
                        result += "```";
                        result
                    } else {
                        format!("Error: Could not open file: {:?}", rdr.err())
                    }
                }
                "wordcount" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let ApplicationCommandInteractionDataOptionValue::String(word) = options {
                        let times = 0;
                        format!("{:?} has been said {} times", word, times)
                    } else {
                        format!("Something went wrong, idk what tho lol TROLD")
                    }
                }
                _ => translate::test_translate("Hello Mr. Pog!").await,
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        if embed {
                            msg.create_embed(|e| {
                                e.title(title)
                                    .description(content)
                                    //.fields(fields)
                            })
                        } else {
                            msg.content(content)
                        }
                    })
            }).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id = std::env::var("DISCORD_APP_ID").expect("Expected an application id in the environment")
        .parse().expect("Application ID is not a valid ID");

    let mut client = Client::builder(&token).event_handler(Handler)
        .application_id(application_id).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
