use super::*;

pub struct Hello;

#[async_trait]
impl CustomCommand for Hello {
    const NAME: &'static str = "hello";

    fn command() -> CreateApplicationCommand {
        CreateApplicationCommand::default()
            .name(Self::NAME)
            .description("Says hello to you")
            .to_owned()
    }

    async fn slash(
        ctx: Context,
        interaction: application_command::ApplicationCommandInteraction,
    ) -> Result<()> {
        interaction
            .create_interaction_response(&ctx, |r| {
                r.interaction_response_data(|r| {
                    r.content(format!("Hello {}!", interaction.user.mention()))
                        .ephemeral(true)
                })
            })
            .await?;
        Ok(())
    }
}
