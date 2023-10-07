use super::*;
use serenity::json::json;

pub struct Purge;

#[async_trait]
impl CustomCommand for Purge {
    const NAME: &'static str = "purge";

    fn command() -> CreateApplicationCommand {
        CreateApplicationCommand::default()
            .name(Self::NAME)
            .description("Purges a specific amount of messages")
            .create_option(|o| {
                o.name("amount")
                    .description("Amount of messages to purge")
                    .kind(command::CommandOptionType::Integer)
                    .required(true)
            })
            .to_owned()
    }

    async fn slash(
        ctx: Context,
        command: application_command::ApplicationCommandInteraction,
    ) -> Result<()> {
        let amount = &command.data.options.get(0).expect("no option found").value;

        let response = match amount {
            Some(value) => {
                let channel_id = command.channel_id.as_u64();

                let messages = ctx
                    .http
                    .get_messages(
                        channel_id.clone(),
                        format!("?limit={}", value.clone().as_u64().unwrap()).as_str(),
                    )
                    .await?;

                match messages.len() {
                    1 => {
                        ctx.http
                            .delete_message(
                                channel_id.to_owned(),
                                messages
                                    .first()
                                    .expect("No messages in match of 1")
                                    .id
                                    .as_u64()
                                    .to_owned(),
                            )
                            .await?;
                        "One message removed".to_string()
                    }
                    _ => {
                        let message_ids = messages
                            .iter()
                            .map(|m| m.id.as_u64().to_owned())
                            .collect::<Vec<u64>>();

                        if let Err(err) = ctx
                            .http
                            .delete_messages(
                                channel_id.to_owned(),
                                &json!({ "messages": message_ids }),
                            )
                            .await
                        {
                            err.to_string()
                        } else {
                            format!("{} messages removed", value)
                        }
                    }
                }
            }
            None => "No messages removed".to_owned(),
        };

        command
            .create_interaction_response(ctx.http, |r| {
                r.interaction_response_data(|c| c.content(response).ephemeral(true))
            })
            .await?;
        Ok(())
    }
}
