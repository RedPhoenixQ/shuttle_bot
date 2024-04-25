use std::vec;

use super::*;

pub struct Test;

#[async_trait]
impl CustomCommand for Test {
    const NAME: &'static str = "test";

    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME).description("haha")
    }

    async fn slash(ctx: Context, interaction: CommandInteraction) -> Result<()> {
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Modal(
                    CreateModal::new(Self::NAME, "This modal").components(vec![
                        CreateActionRow::InputText(
                            CreateInputText::new(InputTextStyle::Short, "First name", "firstname")
                                .value("yeet")
                                .required(true),
                        ),
                        CreateActionRow::InputText(CreateInputText::new(
                            InputTextStyle::Short,
                            "Last name",
                            "lastname",
                        )),
                        CreateActionRow::InputText(CreateInputText::new(
                            InputTextStyle::Short,
                            "Phone nr",
                            "phone",
                        )),
                    ]),
                ),
            )
            .await?;
        Ok(())
    }

    async fn modal(ctx: Context, submit: ModalInteraction) -> Result<()> {
        let mut verified = Vec::new();
        for row in &submit.data.components {
            for component in &row.components {
                info!("Component in submit: {:?}", component);
                match component {
                    ActionRowComponent::InputText(input) => verified.push(input.value.clone()),
                    _ => {}
                };
            }
        }

        submit
            .create_response(
                &ctx,
                // CreateInteractionResponse::Modal(
                //     CreateModal::new(format!("{}_1", Self::NAME), "Part 2").components(vec![
                //         CreateActionRow::InputText(CreateInputText::new(
                //             InputTextStyle::Paragraph,
                //             "Long text",
                //             "long",
                //         )),
                //     ]),
                // ),
                // CreateInteractionResponse::Acknowledge,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("You typed: {:?}", verified)),
                ),
            )
            .await?;
        Ok(())
    }
}
