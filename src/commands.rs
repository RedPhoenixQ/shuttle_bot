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
#[cfg(debug_assertions)]
pub mod test;
pub mod tictactoe;
pub mod week_planner;

pub fn command_list() -> Vec<CreateCommand> {
    vec![
        #[cfg(debug_assertions)]
        test::Test::command(),
        hello::Hello::command(),
        meow::Meowify::command(),
        purge::Purge::command(),
        smashorpass::SmashOrPass::command(),
        tictactoe::TicTacToe::command(),
        week_planner::WeekPlanner::command(),
    ]
}

pub async fn handle_interaction(ctx: Context, interaction: Interaction) -> Result<()> {
    let name = match &interaction {
        Interaction::Command(command) => command.data.name.as_str(),
        Interaction::Component(component) => component
            .data
            .custom_id
            .split_once("_")
            .and_then(|(s, _)| Some(s))
            .unwrap_or(&component.data.custom_id),
        Interaction::Modal(submit) => submit
            .data
            .custom_id
            .split_once("_")
            .and_then(|(s, _)| Some(s))
            .unwrap_or(&submit.data.custom_id),
        Interaction::Autocomplete(command) => command.data.name.as_str(),
        Interaction::Ping(_ping) => todo!(),
        _ => todo!(),
    };

    match name {
        #[cfg(debug_assertions)]
        test::Test::NAME => test::Test::handle_interaction(ctx, interaction).await,
        hello::Hello::NAME => hello::Hello::handle_interaction(ctx, interaction).await,
        meow::Meowify::NAME => meow::Meowify::handle_interaction(ctx, interaction).await,
        purge::Purge::NAME => purge::Purge::handle_interaction(ctx, interaction).await,
        smashorpass::SmashOrPass::NAME => {
            smashorpass::SmashOrPass::handle_interaction(ctx, interaction).await
        }
        tictactoe::TicTacToe::NAME => {
            tictactoe::TicTacToe::handle_interaction(ctx, interaction).await
        }
        week_planner::WeekPlanner::NAME => {
            week_planner::WeekPlanner::handle_interaction(ctx, interaction).await
        }
        _ => Err(anyhow!("No handler found for {}:\n{:?}", name, interaction)),
    }
}

#[allow(unused_variables)]
#[async_trait]
pub trait CustomCommand {
    /// Must be all lowercase for application commands
    const NAME: &'static str;
    fn command() -> CreateCommand;

    async fn handle_interaction(ctx: Context, interaction: Interaction) -> Result<()> {
        match interaction {
            Interaction::Command(command) => Self::slash(ctx, command),
            Interaction::Component(component) => Self::component(ctx, component),
            Interaction::Modal(submit) => Self::modal(ctx, submit),
            Interaction::Autocomplete(autocomplete) => todo!(),
            Interaction::Ping(ping) => todo!(),
            _ => todo!(),
        }
        .await
    }

    async fn component(ctx: Context, component: ComponentInteraction) -> Result<()> {
        Err(anyhow!("Component not implemented for {}", Self::NAME))
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        Err(anyhow!("Slash not implemented for {}", Self::NAME))
    }

    async fn modal(ctx: Context, submit: ModalInteraction) -> Result<()> {
        Err(anyhow!("Modal not implemented for {}", Self::NAME))
    }
}

// #[async_trait]
// pub trait InteractionHandler {
//     fn command_list(&self) -> Vec<CreateCommand>;
//     async fn handle_interaction(&self, ctx: Context, interaction: Interaction);
// }
