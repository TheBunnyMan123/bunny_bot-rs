use poise::serenity_prelude as serenity;
use types::{Data, Error};

mod commands;
mod types;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            let error_message = format!("{}", error);

            let embed = serenity::builder::CreateEmbed::default().title("Error While Running Command")
                .description(error_message)
                .color(0xFF0000);

            if let Err(e) = ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true)).await {
                eprintln!("Error sending error message: {}", e);
            }
        },
        poise::FrameworkError::ArgumentParse { error, ctx, .. } => {
            let error_message = format!("Failed to parse argument {}", error);
            
            let embed = serenity::builder::CreateEmbed::default().title("Invalid Input")
                .description(error_message)
                .color(0xFFFF00);

            if let Err(e) = ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true)).await {
                eprintln!("Error sending error message: {}", e);
            }
        },
        _ => {
            eprintln!("Unhandled framework error: {}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    // Setup the bot
    let token = std::env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN environment variable");
    let intents = serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::roll_dice::roll_dice()],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|context, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

