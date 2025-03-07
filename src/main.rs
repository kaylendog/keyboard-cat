use std::process;

use poise::{serenity_prelude as serenity, FrameworkError};
use songbird::SerenityInit;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod commands;
mod context;
mod provider;
mod resolver;
mod session;
mod util;

use commands::{leave, play, queue, search};
use context::{Error, InstanceContext};

/// Handle errors that occur in the framework.
async fn handle_error(err: FrameworkError<'_, InstanceContext, Box<Error>>) {
    match err {
        // the user's fault
        FrameworkError::ArgumentParse { ctx, .. } => {
            let _ = ctx
                .reply(":x: Error parsing arguments, please try again.")
                .await;
        }
        FrameworkError::CommandCheckFailed { error, ctx, .. } => {
            if let Some(e) = error {
                if e.is_user_facing() {
                    let _ = ctx.reply(format!(":x: {e}")).await;
                }
            }
        }
        FrameworkError::SubcommandRequired { ctx, .. } => {
            let _ = ctx.reply(":x: Please specify a subcommand.").await;
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            let _ = ctx
                .reply(format!(
                    ":x: You are on cooldown. Please wait {}.",
                    humantime::format_duration(remaining_cooldown)
                ))
                .await;
        }
        // not the user's fault
        FrameworkError::Command { ctx, .. } => {
            let _ = ctx.reply(":warning: Encountered an error while running the command. Please report this to @kaylen!").await;
        }
        // trivial errors that we don't need to log
        FrameworkError::NsfwOnly { .. }
        | FrameworkError::GuildOnly { .. }
        | FrameworkError::DmOnly { .. }
        | FrameworkError::NotAnOwner { .. } => {}
        // oh shit
        FrameworkError::CommandPanic { payload, .. } => {
            error!(?payload, "Fatal command panic");
            process::exit(-1);
        }
        _ => {
            error!(%err, "Unhandled framework error");
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(
                    "keyboard_cat=info"
                        .parse()
                        .expect("default directive is invalid"),
                )
                .from_env_lossy(),
        )
        .init();

    info!(
        version = env!("CARGO_PKG_VERSION"),
        sha = env!("VERGEN_GIT_SHA"),
        build_date = env!("VERGEN_BUILD_DATE")
    );

    // framework config
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // initialise framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![play(), leave(), queue(), search()],
            on_error: |err| Box::pin(handle_error(err)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .map_err(|e| Box::new(Error::SerenityError(e)))?;
                Ok(InstanceContext::init().await?)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;

    client.unwrap().start().await.unwrap();
}
