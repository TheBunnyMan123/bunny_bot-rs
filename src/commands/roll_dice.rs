use poise::serenity_prelude as serenity;
use tyche::{dice::{roller::FastRand as FastRandRoller}, Expr, expr::{CalcError}};
use crate::types::{Error, Context};

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Rolls some dice using dice notation"),
    guild_only = false
)]
pub async fn roll_dice(ctx: Context<'_>, #[description = "The dice equation to roll"] equation: String) -> Result<(), Error> {
    let mut roller = FastRandRoller::default();
    let expr = &equation.parse::<Expr>().map_err(|e: tyche::parse::Error| -> Error {format!("Invalid dice notation: {}\n\n**Tips:**\n- Values greater than 255 are not supported - this is a limitation with the typesrary being used", e).into()})?;
    
    let rolled = match expr.eval(&mut roller) {
        Ok(val) => val,
        Err(e) => {
        let e_msg = e.to_string();

        let message = if let Some(first_colon_space_idx) = &e_msg.find(": ") {
            e_msg[first_colon_space_idx + 2..].to_string()
        } else {
            e_msg
        };

        return Err(format!("Error rolling the dice: {}", message).into())
    }};

    let result = rolled.calc().map_err(|e: CalcError| -> Error {format!("Error while calculating result: {}", e).into()});
    let embed = serenity::builder::CreateEmbed::default()
        .title("Rolled your dice!")
        .field("Equation", &equation, true)
        .field("Result", result.unwrap().to_string(), true)
        .field("", "Library: [tyche-rs](https://github.com/Gawdl3y/tyche-rs) ([License](https://github.com/Gawdl3y/tyche-rs/blob/main/LICENSE.md))", false)
        .color(0xFFAAEE);

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .reply(true)).await?;
    Ok(())
}

