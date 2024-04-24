use super::*;

pub struct Hello;

#[async_trait]
impl CustomCommand for Hello {
    const NAME: &'static str = "hello";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .description("Says hello to you")
            .to_owned()
    }

    async fn slash(ctx: Context, interaction: CommandInteraction) -> Result<()> {
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Hello {}!", interaction.user.mention()))
                        .ephemeral(true),
                ),
            )
            .await?;
        Ok(())
    }
}
