use super::*;

pub struct SmashOrPass;

#[async_trait]
impl CustomCommand for SmashOrPass {
    const NAME: &'static str = "smashorpass";

    fn command() -> CreateApplicationCommand {
        CreateApplicationCommand::default()
            .name(Self::NAME)
            .create_option(|o| {
                o.name("name")
                    .description("Name of candidate")
                    .kind(command::CommandOptionType::String)
                    .required(true)
            })
            .description("Provide a name to smash or pass")
            .to_owned()
    }

    async fn slash(
        ctx: Context,
        command: application_command::ApplicationCommandInteraction,
    ) -> Result<()> {
        command
            .create_interaction_response(&ctx, |response| {
                if let Some(candidate) = command
                    .data
                    .options
                    .get(0)
                    .and_then(|option| option.value.as_ref().and_then(|v| v.as_str()))
                {
                    response.interaction_response_data(|msg| {
                        msg.content(
                            MessageBuilder::default()
                                .push("Smash or Pass: ")
                                .push_bold(candidate)
                                .build(),
                        )
                    })
                } else {
                    response.interaction_response_data(|msg| {
                        msg.content("No candidate given").ephemeral(true)
                    })
                }
            })
            .await
            .unwrap();

        let response = command.get_interaction_response(&ctx).await?;
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
