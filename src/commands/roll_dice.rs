use poise::serenity_prelude as serenity;
use tyche::{dice::{roller::FastRand as FastRandRoller}, Expr, expr::{Evaled, CalcError}};
use crate::types::{Error, Context};

#[poise::command(
    slash_command,
    description_localized("en-US", "Rolls some dice using dice notation"),
    guild_only = false
)]
pub async fn roll_dice(ctx: Context<'_>, equation: String) -> Result<(), Error> {
    let mut roller = FastRandRoller::default();
    let expr = equation.parse::<Expr>().map_err(|e: tyche::parse::Error| -> Error {format!("Invalid dice notation: {}", e).into()})?;
    let rolled: Result<Evaled, _> = expr.eval(&mut roller).map_err(|e| -> Error {format!("Error while rolling dice: {}", e).into()});
    let result = rolled.unwrap().calc().map_err(|e: CalcError| -> Error {format!("Error while calculating result: {}", e).into()});

    let embed = serenity::builder::CreateEmbed::default()
        .title("Rolled your dice!")
        .field("Equation", &equation, true)
        .field("Result", result.unwrap().to_string(), true)
        .field("", "Library: [tyche-rs](https://github.com/Gawdl3y/tyche-rs) ([License](https://github.com/Gawdl3y/tyche-rs/blob/main/LICENSE.md))", false)
        .color(0xFFAAEE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

