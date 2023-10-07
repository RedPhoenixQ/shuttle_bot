pub use anyhow::{anyhow, bail, Context as AnyhowContext, Result};
pub use serenity::async_trait;
pub use serenity::builder::*;
pub use serenity::model::prelude::*;
pub use serenity::prelude::*;
pub use serenity::utils::*;
pub use tracing::{error, info};

pub mod hello;
pub mod meow;
pub mod purge;
pub mod smashorpass;
pub mod tictactoe;

macro_rules! impl_interaction_handler {
    ($($cmd:ty),+) => {
        pub fn command_list() -> Vec<serenity::builder::CreateApplicationCommand> {
            vec![
                $(<$cmd>::command(),)+
            ]
        }

        pub async fn handle_interaction(ctx: Context, interaction: Interaction) -> Result<()> {
            match interaction {
                Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                    $(<$cmd>::NAME => <$cmd>::slash(ctx, command).await,)+
                    _ => Err(anyhow!("Unknown application_command {}: {:?}", command.data.name, command)),
                }
                Interaction::MessageComponent(component) => match component.data.custom_id.split_once("_").with_context(|| format!("Could not parse {:?}: {:?}", component.data.custom_id, component))?.0
                {
                    $(<$cmd>::NAME => <$cmd>::component(ctx, component).await,)+
                    _ => Err(anyhow!("Unknown message_component {}: {:?}", component.data.custom_id, component)),
                },
                #[allow(unused_variables)]
                Interaction::ModalSubmit(submit) => todo!(),
                #[allow(unused_variables)]
                Interaction::Autocomplete(autocomplete) => todo!(),
                #[allow(unused_variables)]
                Interaction::Ping(ping) => todo!(),
            }
        }
    };
}

impl_interaction_handler!(
    hello::Hello,
    meow::Meowify,
    purge::Purge,
    smashorpass::SmashOrPass,
    tictactoe::TicTacToe
);

#[allow(unused_variables)]
#[async_trait]
pub trait CustomCommand {
    /// Must be all lowercase for application commands
    const NAME: &'static str;
    fn command() -> CreateApplicationCommand;

    async fn component(
        ctx: Context,
        component: message_component::MessageComponentInteraction,
    ) -> Result<()> {
        Err(anyhow!("Component not implemented for {}", Self::NAME))
    }

    async fn slash(
        ctx: Context,
        command: application_command::ApplicationCommandInteraction,
    ) -> Result<()> {
        Err(anyhow!("Slash not implemented for {}", Self::NAME))
    }
}

// #[async_trait]
// pub trait InteractionHandler {
//     fn command_list(&self) -> Vec<CreateApplicationCommand>;
//     async fn handle_interaction(&self, ctx: Context, interaction: Interaction);
// }
