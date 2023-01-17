use rand::seq::SliceRandom;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn lenny(
    ctx: Context<'_>,
    #[description = "Message"] message: Option<String>,
) -> Result<(), Error> {
    let lennys = vec![
    "( \u{0361}\u{00B0} \u{035C}\u{0296} \u{0361}\u{00B0})",
    "( \u{0360}\u{00B0} \u{035F}\u{0296} \u{0361}\u{00B0})",
    "\u{1566}( \u{0361}\u{b0} \u{035c}\u{0296} \u{0361}\u{b0})\u{1564}",
    "( \u{0361}\u{00B0} \u{035C}\u{0296} \u{0361}\u{00B0})",
    "( \u{0361}~ \u{035C}\u{0296} \u{0361}\u{00B0})",
    "( \u{0361}o \u{035C}\u{0296} \u{0361}o)", "\u{0361}\u{00B0} \u{035C}\u{0296} \u{0361} -",
    "( \u{0361}\u{0361} \u{00B0} \u{035C} \u{0296} \u{0361} \u{00B0})\u{FEFF}",
    "( \u{0361} \u{0361}\u{00B0} \u{0361}\u{00B0}  \u{0296} \u{0361}\u{00B0} \u{0361}\u{00B0})",
    "(\u{0E07} \u{0360}\u{00B0} \u{035F}\u{0644}\u{035C} \u{0361}\u{00B0})\u{0E07}",
    "( \u{0361}\u{00B0} \u{035C}\u{0296} \u{0361} \u{00B0})",
    "( \u{0361}\u{00B0}\u{256D}\u{035C}\u{0296}\u{256E}\u{0361}\u{00B0} )"
    ];

    let m = message.unwrap_or_else(|| "".to_string());
    let response = format!("{} {}", lennys.choose(&mut rand::thread_rng()).unwrap(), m);
    ctx.say(response).await?;
    Ok(())
}
