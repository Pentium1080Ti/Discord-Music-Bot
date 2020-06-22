![Discord-Music-Bot](https://github.com/Pentium1080Ti/Discord-Music-Bot/workflows/Discord-Music-Bot/badge.svg)

# Discord Music Bot

A simple music bot written in Rust. It uses the [serenity](https://github.com/serenity-rs/serenity) wrapper for the DAPI.

## Features

- [x] Play and stop youtube songs
- [ ] Queue multiple songs at once
- [ ] Use spotify to play songs

## Installation

First, clone the repository with
```
git clone https://github.com/Pentium1080Ti/Discord-Music-Bot.git
``` 

Then cd into the directory

```
cd Discord-Music-Bot/
```

Finally compile a binary using cargo

```
cargo build --release
```

The binary is in `target/release/discord_music_bot`

## Setup

In order for the bot to run, you need to add your [token](https://github.com/Pentium1080Ti/Discord-Music-Bot/blob/4a72d6e2a8ca58facaabece03eb31f67c5ed3150/src/main.rs#L38) and a [prefix](https://github.com/Pentium1080Ti/Discord-Music-Bot/blob/4a72d6e2a8ca58facaabece03eb31f67c5ed3150/src/main.rs#L48) to the bot.

## Commands

- `<prefix>join`
- `<prefix>leave`
- `<prefix>play [url]`
- `<prefix>stop`
