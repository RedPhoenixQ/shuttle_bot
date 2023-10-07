pub use anyhow::{anyhow, bail, Context as AnyhowContext, Result};
pub use serenity::async_trait;
pub use serenity::builder::*;
pub use serenity::model::prelude::*;
pub use serenity::prelude::*;
pub use serenity::utils::*;
pub use tracing::{error, info};

use crate::commands::*;

#[allow(unused_variables)]
#[async_trait]
pub trait ReactionHandler {
    async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()>;
}

pub async fn handle_reaction_add(ctx: Context, reaction: Reaction) {
    for handler in [meow::Meowify::reaction_add] {
        match handler(&ctx, &reaction).await {
            Err(err) => error!("{}", err),
            _ => {}
        };
    }
}
