use super::*;

pub struct SmashOrPass;

#[async_trait]
impl CustomCommand for SmashOrPass {
    const NAME: &'static str = "smashorpass";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "name", "Name of candidate")
                    .required(true),
            )
            .description("Provide a name to smash or pass")
            .to_owned()
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        command
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    if let Some(candidate) = command
                        .data
                        .options
                        .get(0)
                        .and_then(|option| option.value.as_str())
                    {
                        CreateInteractionResponseMessage::new().content(
                            MessageBuilder::default()
                                .push("Smash or Pass: ")
                                .push_bold(candidate)
                                .build(),
                        )
                    } else {
                        CreateInteractionResponseMessage::new()
                            .content("No candidate given")
                            .ephemeral(true)
                    },
                ),
            )
            .await
            .unwrap();

        let response = command.get_response(&ctx).await?;
        println!("response in callback {:?}", &response);
        let smash_react = response.react(&ctx, '🥵').await;
        let pass_react = response.react(&ctx, '😒').await;

        match (smash_react, pass_react) {
            (Ok(_), Ok(_)) => {
                println!("Added reactions to 'smashorpass' message");
                Ok(())
            }
            _ => Err(anyhow!("Failed to reactions to 'smashorpass' message")),
        }
    }
}
