use crate::reactions::ReactionHandler;

use super::*;

pub struct Meowify;

const CAT_SMIRK: &str = "😼";

#[async_trait]
impl CustomCommand for Meowify {
    const NAME: &'static str = "😼 Meowify";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .kind(CommandType::Message)
            .to_owned()
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        // dbg!(&command.data);

        if let Some(msg) = command
            .data
            .resolved
            .messages
            .get(&command.data.target_id.unwrap_or(0.into()).into())
        {
            command
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(meowify(&msg.content)),
                    ),
                )
                .await?;
        } else {
            eprintln!("Could not find msg for interaction: {:?}", command);
        }
        Ok(())
    }
}

#[async_trait]
impl ReactionHandler for Meowify {
    async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()> {
        if reaction.emoji.unicode_eq(CAT_SMIRK)
            && reaction.member.as_ref().is_some_and(|m| m.user.bot)
        {
            let message = reaction.message(&ctx).await?;
            message.reply(&ctx, meowify(&message.content)).await?;
        }
        Ok(())
    }
}

fn meowify(text: &str) -> String {
    let mut meow = "meow".chars().cycle();
    let meow_text: String = text
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let next = meow.next().unwrap();
                if c.is_ascii_uppercase() {
                    next.to_uppercase().take(1).next().unwrap()
                } else {
                    next
                }
            } else {
                c
            }
        })
        .collect();
    meow_text.replace(".", &format!(". {}", CAT_SMIRK))
}
