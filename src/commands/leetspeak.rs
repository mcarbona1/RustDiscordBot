use crate::{Context, Error};
use rand::seq::SliceRandom;

// Mappings between normal and leetspeek
const A: [char; 5] = ['a', '4', '4', '@', '@'];
const C: [char; 5] = ['c', 'c', 'c', '(', '<'];
const E: [char; 4] = ['e', '3', '3', '3'];
const L: [char; 5] = ['l', '1', '1', '1', '|'];
const O: [char; 4] = ['o', '0', '0', '0'];
const S: [char; 5] = ['s', '5', '5', '$', 'z'];
const T: [char; 5] = ['t', '7', '+', '7', '+'];

#[poise::command(slash_command, prefix_command)]
pub async fn leet(
    ctx: Context<'_>,
    #[description = "Message"] message: String,
) -> Result<(), Error> {
    let mut response = "".to_string();
    for letter in message.chars() {
        if letter == 'a' {
            response.push(A.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 'c' {
            response.push(C.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 'e' {
            response.push(E.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 'l' {
            response.push(L.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 'o' {
            response.push(O.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 's' {
            response.push(S.choose(&mut rand::thread_rng()).unwrap().clone());
        } else if letter == 't' {
            response.push(T.choose(&mut rand::thread_rng()).unwrap().clone());
        } else {
            response.push(letter);
        }

    }

    ctx.say(response).await?;
    Ok(())
}
