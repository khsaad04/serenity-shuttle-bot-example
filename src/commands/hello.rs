#![allow(deprecated)]
use serenity::all::{
    prelude::*,
    standard::{macros::command, CommandResult},
    Message,
};

#[command]
pub async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Hello there").await?;
    Ok(())
}
