#![allow(dead_code)]
#![allow(unused_variables)]

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::prelude::{GuildStatus, GuildUnavailable, GuildId, User, CurrentUser, VoiceState};
use serenity::model::event::{ChannelPinsUpdateEvent, GuildMembersChunkEvent, GuildMemberUpdateEvent, InviteCreateEvent, InviteDeleteEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent, ThreadListSyncEvent, ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent};
use serenity::model::channel::{Reaction, ReactionType, Channel, GuildChannel, ChannelCategory, StageInstance, PartialGuildChannel};
use serenity::model::id::{ApplicationId, ChannelId, EmojiId, IntegrationId, MessageId, RoleId};
use std::borrow::Borrow;
use std::collections::HashMap;
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::model::gateway::Presence;
use serenity::model::guild::{Emoji, Guild, Integration, Member, PartialGuild, Role, ThreadMember};
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::{Interaction, InteractionResponseType};

struct Handler;
static MAIN_GUILD_ID: u64 = 745725474465906732;
static MAIN_GUILD: GuildStatus = GuildStatus::Offline(GuildUnavailable { id: GuildId{ 0: MAIN_GUILD_ID }, unavailable: true });
static HAHAYES_EMOTE: u64 = 627151632534339595;
static MOG_EMOTE: u64 = 745729242423099585;
static THEFLIP_EMOTE: u64 = 758463821831471174;
static IRCH_UID: u64 = 292362225388355584;
static MARK_UID: u64 = 179024507657256960;

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
            commands.create_application_command(|command| {
                command.name("cool").description("It is a cool command!")
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
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "cool" => "YOUR MOM".to_string(),
                _ => "Unknown".to_string(),
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content(content))
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
    println!("Hello, world!");
}
