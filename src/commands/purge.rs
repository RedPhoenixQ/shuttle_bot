use super::*;
use serenity::json::json;

pub struct Purge;

#[async_trait]
impl CustomCommand for Purge {
    const NAME: &'static str = "purge";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .description("Purges a specific amount of messages")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "amount",
                    "Amount of messages to purge",
                )
                .required(true),
            )
            .to_owned()
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        let amount = command
            .data
            .options
            .first()
            .expect("amount option to exist")
            .value
            .as_i64()
            .map(|n| (n % u8::MAX as i64) as u8);

        let messages = ctx
            .http
            .get_messages(command.channel_id, None, amount)
            .await?;

        let msg = match messages.len() {
            1 => {
                ctx.http
                    .delete_message(
                        command.channel_id,
                        messages.first().expect("No messages in match of 1").id,
                        Some("Purged by purge command"),
                    )
                    .await?;
                "One message removed".to_string()
            }
            _ => {
                let message_ids = messages
                    .iter()
                    .map(|m| m.id.to_string())
                    .collect::<Vec<_>>();

                if let Err(err) = ctx
                    .http
                    .delete_messages(
                        command.channel_id,
                        &json!({ "messages": message_ids }),
                        Some("Purged by purge command"),
                    )
                    .await
                {
                    err.to_string()
                } else {
                    format!(
                        "{} messages removed",
                        amount
                            .expect("amount to be Some because more than 0 messsages were fetched")
                    )
                }
            }
        };

        command
            .create_response(
                ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(msg)
                        .ephemeral(true),
                ),
            )
            .await?;
        Ok(())
    }
}
