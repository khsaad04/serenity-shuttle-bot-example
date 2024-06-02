#![allow(deprecated)]
mod commands;

use anyhow::Context as _;
use serenity::all::{
    async_trait,
    model::gateway::Ready,
    prelude::*,
    standard::{macros::group, Configuration},
    StandardFramework,
};
use shuttle_runtime::SecretStore;
use tracing::info;

use crate::commands::hello::*;
use crate::commands::help::*;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(hello)]
struct General;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);
    framework.configure(
        Configuration::new()
            .case_insensitivity(true)
            .with_whitespace(true)
            .prefix(">"),
    );
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
