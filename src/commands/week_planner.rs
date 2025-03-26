use super::*;

pub struct WeekPlanner;

#[async_trait]
impl CustomCommand for WeekPlanner {
    const NAME: &'static str = "Week Planner";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .kind(CommandType::Message)
            .to_owned()
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        let Some(ResolvedTarget::Message(msg)) = command.data.target() else {
            bail!("Could not find msg for interaction: {:?}", command);
        };

        command
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("New week planner")
                        .ephemeral(true),
                ),
            )
            .await?;
        for day in [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ] {
            let follow_up = command
                .channel_id
                .send_message(&ctx, CreateMessage::new().content(day))
                .await?;
            for reaction in &msg.reactions {
                follow_up
                    .react(&ctx, reaction.reaction_type.clone())
                    .await?;
            }
        }
        Ok(())
    }
}
