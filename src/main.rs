use std::{sync::Arc};
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::{client::Context, prelude::Mutex};
use serenity::{
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready, misc::Mentionable},
    Result as SerenityResult,
    voice,
};
use serenity::prelude::*;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("connected as {}", ready.user.name);
    }
}

#[group]
#[commands(join, leave, play, stop)]
struct General;

fn main() {
    let mut client = Client::new("bot token goes here", Handler)
        .expect("failed to create client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix("."))
        .group(&GENERAL_GROUP));

    let _ = client.start().map_err(|e| println!("failed to start client: {:?}", e));
}

#[command]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "DMs are not supported"));

            return Ok(());
        },
    };

    let guild_id = guild.read().id;

    let channel_id = guild.read().voice_states.get(&msg.author.id).and_then(|x| x.channel_id);


    let connect = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("expected voicemanager in sharemap.");
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect).is_some() {
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect.mention())));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Failed to join the channel"));
    }

    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "DMs are not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("expected voicemanager in sharemap.");
    let mut manager = manager_lock.lock();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id);
        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel"));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "I am not in a voice channel"));
    }

    Ok(())
}

#[command]
fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_e) => {
            check_msg(msg.channel_id.say(&ctx.http, "Please provide a URL"));
            return Ok(());
        },
    };

    if !url.starts_with("http")  {
        check_msg(msg.channel_id.say(&ctx.http, "Invalid URL"));
        
        return Ok(());
    }

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "failed to get channel info"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("expected voicemanager in sharemap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl(&url) {
            Ok(source) => source,
            Err(e) => {
                println!("failed to start playback: {:?}", e);
                check_msg(msg.channel_id.say(&ctx.http, "Failed to start playback"));

                return Ok(());
            },
        };
        handler.play(source);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Please join a voice channel first"));
    }

    Ok(())
}

#[command]
fn stop(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "failed to get channel info"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("expected voicemanager in sharemap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        handler.stop();
        check_msg(msg.channel_id.say(&ctx.http, "Stopped playback"));
    }

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(e) = result {
        println!("failed to send message: {:?}", e);
    }
}