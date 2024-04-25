#[allow(unused_imports)]
pub use anyhow::{anyhow, bail, Context as AnyhowContext, Result};
#[allow(unused_imports)]
pub use serenity::async_trait;
#[allow(unused_imports)]
pub use serenity::builder::*;
#[allow(unused_imports)]
pub use serenity::model::prelude::*;
#[allow(unused_imports)]
pub use serenity::prelude::*;
#[allow(unused_imports)]
pub use serenity::utils::*;
#[allow(unused_imports)]
pub use tracing::{error, info};

pub mod hello;
pub mod meow;
pub mod purge;
pub mod smashorpass;
pub mod tictactoe;

macro_rules! impl_interaction_handler {
    ($($cmd:ty),+) => {
        pub fn command_list() -> Vec<CreateCommand> {
            vec![
                $(<$cmd>::command(),)+
            ]
        }

        pub async fn handle_interaction(ctx: Context, interaction: Interaction) -> Result<()> {
            match interaction {
                Interaction::Command(command) => match command.data.name.as_str() {
                    $(<$cmd>::NAME => <$cmd>::slash(ctx, command).await,)+
                    _ => Err(anyhow!("Unknown application_command {}: {:?}", command.data.name, command)),
                }
                Interaction::Component(component) => match component.data.custom_id
                    .split_once("_")
                    .and_then(|(s, _)| Some(s))
                    .unwrap_or(&component.data.custom_id)
                {
                    $(<$cmd>::NAME => <$cmd>::component(ctx, component).await,)+
                    _ => Err(anyhow!("Unknown message_component {}: {:?}", component.data.custom_id, component)),
                },
                #[allow(unused_variables)]
                Interaction::Modal(submit) => todo!(),
                #[allow(unused_variables)]
                Interaction::Autocomplete(autocomplete) => todo!(),
                #[allow(unused_variables)]
                Interaction::Ping(ping) => todo!(),
                _ => todo!(),
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
    fn command() -> CreateCommand;

    async fn component(ctx: Context, component: ComponentInteraction) -> Result<()> {
        Err(anyhow!("Component not implemented for {}", Self::NAME))
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        Err(anyhow!("Slash not implemented for {}", Self::NAME))
    }
}

// #[async_trait]
// pub trait InteractionHandler {
//     fn command_list(&self) -> Vec<CreateCommand>;
//     async fn handle_interaction(&self, ctx: Context, interaction: Interaction);
// }
