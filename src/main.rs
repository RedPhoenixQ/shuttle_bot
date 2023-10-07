use anyhow::anyhow;
// use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{async_trait, json::json, model::prelude::GuildId};
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod commands;
mod reactions;

// struct Bot;

// #[async_trait]
// impl EventHandler for Bot {
//     async fn message(&self, ctx: Context, msg: Message) {
//         if msg.content == "!hello" {
//             if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
//                 error!("Error sending message: {:?}", e);
//             }
//         }
//     }

//     async fn ready(&self, _: Context, ready: Ready) {
//         info!("{} is connected!", ready.user.name);
//     }
// }

struct Handler {
    dev_guild_ids: Option<Vec<u64>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let commands = commands::command_list();
        info!("Ready event triggered");
        match &self.dev_guild_ids {
            Some(guildids) => {
                for guild in guildids {
                    GuildId(*guild)
                        .set_application_commands(&ctx.http, |c| {
                            c.set_application_commands(commands.clone())
                        })
                        .await
                        .expect("Could not set commands");
                }

                info!("{} is online in test env!", ready.user.name);
            }
            None => {
                let current_commands = ctx
                    .http
                    .get_global_application_commands()
                    .await
                    .expect("Could not fetch current commands");

                for c in current_commands {
                    if commands.iter().any(|c| c.0.get("name") == c.0.get("name")) {
                        match ctx
                            .http
                            .delete_global_application_command(c.id.as_u64().to_owned())
                            .await
                        {
                            Err(err) => error!("{}", anyhow::format_err!(err)),
                            _ => {}
                        }
                    }
                }

                ctx.http
                    .create_global_application_commands(&json!(commands
                        .iter()
                        .map(|c| c.0.clone())
                        .collect::<Vec<_>>()))
                    .await
                    .expect("Could not set global applications commands");

                info!("{} is online!", ready.user.name);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("interaction_create: {:?}", interaction);
        commands::handle_interaction(ctx, interaction).await;
    }

    // async fn message(&self, ctx: Context, message: Message) {
    //     for handler in [wordgame::WordGame::message] {
    //         match handler(&ctx, &message).await {
    //             Err(err) => eprintln!("{}", err),
    //             _ => {}
    //         };
    //     }
    // }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        info!("reaction_add create: {:?}", reaction);
        reactions::handle_reaction_add(ctx, reaction).await;
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler {
            dev_guild_ids: secret_store.get("DISCORD_GUILD_ID").and_then(|guilds| {
                Some(
                    guilds
                        .split_terminator(',')
                        .filter_map(|id| id.parse::<u64>().ok())
                        .collect::<Vec<_>>(),
                )
            }),
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
