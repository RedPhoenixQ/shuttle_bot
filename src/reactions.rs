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

use crate::commands::*;

#[allow(unused_variables)]
#[async_trait]
pub trait ReactionHandler {
    async fn reaction_add(ctx: &Context, reaction: &Reaction) -> Result<()>;
}

pub async fn handle_reaction_add(ctx: Context, reaction: Reaction) {
    for handler in [meow::Meowify::reaction_add] {
        if let Err(err) = handler(&ctx, &reaction).await {
            error!("{}", err)
        };
    }
}
