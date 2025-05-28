use poise::serenity_prelude as serenity;
use crate::types::{Error, Context};

#[poise::command(
    prefix_command,
    description_localized("en-US", "Displays all commands"),
    guild_only = false
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let mut embed = serenity::builder::CreateEmbed::default()
        .title("Command List")
        .color(0xFFAAEE);

    let commands = &ctx.framework().options().commands;
    for command in commands {
        if !command.hide_in_help {
            let mut command_name = format!("bb!{}", command.name);

            for parameter in &command.parameters {
                command_name.push_str(&format!(" <{}>", parameter.name));
            }

            let command_description = match command.description_localizations.get("en-US") {
                Some(val) => val,
                None => "No description provided"
            };

            embed = embed.field(command_name, command_description, false);
        }
    }

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)).await?;
    Ok(())
}

