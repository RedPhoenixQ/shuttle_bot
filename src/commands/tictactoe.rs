use std::collections::HashMap;
use std::fmt::Display;

use super::*;

#[derive(Debug, Default)]
pub struct TicTacToe {
    state: HashMap<Coord, Tile>,
    winning: Option<Winning>,
    next_turn: Player,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]

enum Player {
    #[default]
    Opponent,
    Challenger,
}

fn next_player(state: &HashMap<Coord, Tile>) -> Player {
    if state
        .iter()
        .filter(|(_, &tile)| tile != Tile::Empty)
        .count()
        % 2
        == 0
    {
        Player::Opponent
    } else {
        Player::Challenger
    }
}

impl TicTacToe {
    fn new(mut state: HashMap<Coord, Tile>, clicked_tile: Option<Coord>) -> Self {
        if let Some(clicked) = clicked_tile {
            state.insert(clicked, next_player(&state).into());
        }
        Self {
            next_turn: next_player(&state),
            winning: calculate_winner(&state),
            state,
        }
    }
}

#[async_trait]
impl CustomCommand for TicTacToe {
    const NAME: &'static str = "TicTacToe";
    fn command() -> CreateCommand {
        CreateCommand::new(Self::NAME)
            .kind(CommandType::User)
            .to_owned()
    }

    async fn slash(ctx: Context, command: CommandInteraction) -> Result<()> {
        if let Some(ResolvedTarget::User(target, _)) = command.data.target() {
            if target.bot {
                command
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("You cannot challenge a bot to TicTacToe!")
                                .ephemeral(true),
                        ),
                    )
                    .await?;
            } else {
                command
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content(
                                    MessageBuilder::default()
                                        .mention(&target.id)
                                        .push_line(" has been challenged to TicTacToe!")
                                        .push(X_EMOJI)
                                        .mention(&target.id)
                                        .push("'s turn")
                                        .build(),
                                )
                                .components(create_components(&TicTacToe::default())),
                        ),
                    )
                    .await?;
            }
            Ok(())
        } else {
            Err(anyhow!("No user for the user command tictactoe"))
        }
    }

    async fn component(ctx: Context, interaction: ComponentInteraction) -> Result<()> {
        let challenger = &interaction
            .message
            .interaction
            .as_ref()
            .ok_or(anyhow!("There was no interaction on the message"))?
            .user;

        // Check if user is not part of the game
        if &interaction.user != challenger
            && !interaction
                .message
                .mentions
                .iter()
                .any(|user| user == &interaction.user)
        {
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("You are not part of this game")
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        };

        // Handle remove game
        if interaction.data.custom_id == REMOVE_ID {
            interaction.message.delete(&ctx).await?;
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("The game has been removed")
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        }

        let opponent = if interaction.message.mentions.len() == 1 {
            interaction
                .message
                .mentions
                .iter()
                .next()
                .ok_or(anyhow!("There was no mentions"))?
        } else {
            interaction
                .message
                .mentions
                .iter()
                .find(|&user| user != challenger)
                .ok_or(anyhow!(
                    "There was no mentions other than the player who created the interaction"
                ))?
        };

        let clicked_coord: Coord = interaction
            .data
            .custom_id
            .as_str()
            .split_once("_")
            .ok_or(anyhow!("Invalid customId, does not have a '_'"))?
            .1
            .try_into()?;

        let game = TicTacToe::new(
            interaction
                .message
                .components
                .iter()
                .flat_map(|row| {
                    row.components.iter().filter_map(|component| {
                        if let ActionRowComponent::Button(button) = component {
                            if let ButtonKind::NonLink { custom_id, .. } = &button.data {
                                let coord = custom_id.split_once("_")?.1.try_into().ok()?;
                                let tile = match &button.emoji {
                                    Some(e) if e.unicode_eq(X_EMOJI) => Tile::X,
                                    Some(e) if e.unicode_eq(O_EMOJI) => Tile::O,
                                    _ => Tile::Empty,
                                };
                                return Some((coord, tile));
                            }
                        }
                        unreachable!();
                    })
                })
                .collect(),
            Some(clicked_coord),
        );

        // It is your turn if we reach here, meaning that next_turn must be your opponents
        if match game.next_turn {
            Player::Challenger => &interaction.user == opponent,
            Player::Opponent => &interaction.user == challenger,
        } {
            let mut msg = MessageBuilder::default();

            // Preserve first line
            msg.push_line(interaction.message.content.split_once("\n").unwrap().0);

            match &game.winning {
                Some(winning) => match winning {
                    Winning::Tie => msg.push("The game is a tie"),
                    _ => msg
                        .push(
                            Tile::from(match game.next_turn {
                                Player::Opponent => Player::Challenger,
                                Player::Challenger => Player::Opponent,
                            })
                            .to_string(),
                        )
                        .mention(match game.next_turn {
                            Player::Opponent => &challenger.id,
                            Player::Challenger => &opponent.id,
                        })
                        .push(" is the winner!"),
                },
                None => msg
                    .push(Tile::from(game.next_turn).to_string())
                    .mention(match game.next_turn {
                        Player::Challenger => &challenger.id,
                        Player::Opponent => &opponent.id,
                    })
                    .push("'s turn"),
            };
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .content(msg.build())
                            .components(create_components(&game)),
                    ),
                )
                .await?;
        } else {
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .content("Its not your turn")
                            .ephemeral(true),
                    ),
                )
                .await?;
        };
        Ok(())
    }
}

const COMPONENT_ROWS: [Row; 3] = [Row::Bottom, Row::Middle, Row::Top];
const COMPONENT_COLUMNS: [Column; 3] = [Column::Left, Column::Center, Column::Right];

const REMOVE_ID: &str = const_format::formatcp!("{}_{}", TicTacToe::NAME, "_remove");

const X_EMOJI: &str = "❌";
const O_EMOJI: &str = "⭕";
const EMPTY_EMOJI: &str = "⬛";

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Tile {
    #[default]
    Empty,
    X,
    O,
}

impl From<Player> for Tile {
    fn from(value: Player) -> Self {
        match value {
            Player::Challenger => Tile::O,
            Player::Opponent => Tile::X,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tile::X => X_EMOJI,
            Tile::O => O_EMOJI,
            Tile::Empty => EMPTY_EMOJI,
        })
    }
}

#[derive(Debug)]
enum Winning {
    Vertical(Column),
    Horizontal(Row),
    Diagonal(Diagonal),
    Tie,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Row {
    Top,
    Middle,
    Bottom,
}

impl TryFrom<&str> for Row {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "top" => Ok(Self::Top),
            "middle" => Ok(Self::Middle),
            "bottom" => Ok(Self::Bottom),
            _ => Err(anyhow!("Could not parse Row from String: {}", value)),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Row::Top => "top",
            Row::Middle => "middle",
            Row::Bottom => "bottom",
        })
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Column {
    Left,
    Center,
    Right,
}

impl TryFrom<&str> for Column {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Err(anyhow!("Could not parse Column from String: {}", value)),
        }
    }
}
impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Column::Left => "left",
            Column::Center => "center",
            Column::Right => "right",
        })
    }
}

#[derive(Debug, PartialEq)]
enum Diagonal {
    TopLeftToBottomRight,
    BottomLeftToTopRight,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord(Row, Column);

impl TryFrom<&str> for Coord {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (col, row) = value
            .split_once("_")
            .ok_or(anyhow!("Malformed input for Coords: {}", value))?;
        Ok(Coord(col.try_into()?, row.try_into()?))
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}_{}_{}", TicTacToe::NAME, self.0, self.1))?;
        Ok(())
    }
}

fn calculate_winner(state: &HashMap<Coord, Tile>) -> Option<Winning> {
    if !state.iter().any(|(_, &tile)| tile == Tile::Empty) {
        return Some(Winning::Tie);
    }

    if let Some(winning) = COMPONENT_COLUMNS.iter().find_map(|&column| {
        match state
            .iter()
            .filter(|(k, _)| k.1 == column)
            .map(|(_, v)| v)
            .reduce(|a, v| if a == v { v } else { &Tile::Empty })
        {
            None | Some(Tile::Empty) => None,
            _ => Some(Winning::Vertical(column)),
        }
    }) {
        return Some(winning);
    };
    if let Some(winning) = COMPONENT_ROWS.iter().find_map(|&row| {
        match state
            .iter()
            .filter(|(k, _)| k.0 == row)
            .map(|(_, v)| v)
            .reduce(|a, v| if a == v { v } else { &Tile::Empty })
        {
            None | Some(Tile::Empty) => None,
            _ => Some(Winning::Horizontal(row)),
        }
    }) {
        return Some(winning);
    };

    match (
        state.get(&Coord(Row::Middle, Column::Center)),
        state.get(&Coord(Row::Top, Column::Left)),
        state.get(&Coord(Row::Bottom, Column::Right)),
        state.get(&Coord(Row::Bottom, Column::Left)),
        state.get(&Coord(Row::Top, Column::Right)),
    ) {
        (Some(a), Some(b), Some(c), _, _) if a != &Tile::Empty && a == b && a == c => {
            Some(Winning::Diagonal(Diagonal::TopLeftToBottomRight))
        }
        (Some(a), _, _, Some(b), Some(c)) if a != &Tile::Empty && a == b && a == c => {
            Some(Winning::Diagonal(Diagonal::BottomLeftToTopRight))
        }
        _ => None,
    }
}

fn create_components(game: &TicTacToe) -> Vec<CreateActionRow> {
    COMPONENT_ROWS
        .into_iter()
        .map(|row| {
            CreateActionRow::Buttons(
                COMPONENT_COLUMNS
                    .into_iter()
                    .map(|col| {
                        let coord = Coord(row, col);
                        let tile = game.state.get(&coord).unwrap_or(&Tile::Empty);
                        CreateButton::new(coord.to_string())
                            .disabled(*tile != Tile::Empty || game.winning.is_some())
                            .style(match &game.winning {
                                Some(value) => get_style(&coord, value),
                                None => ButtonStyle::Secondary,
                            })
                            .emoji(ReactionType::Unicode(tile.to_string()))
                    })
                    .collect(),
            )
        })
        .chain(std::iter::once(CreateActionRow::Buttons(vec![
            CreateButton::new(REMOVE_ID)
                .label("Remove")
                .style(ButtonStyle::Danger),
        ])))
        .collect()
}

fn get_style(id: &Coord, value: &Winning) -> ButtonStyle {
    if match value {
        Winning::Vertical(col) => id.1 == *col,
        Winning::Horizontal(row) => id.0 == *row,
        Winning::Diagonal(diagonal) => match id {
            &Coord(Row::Middle, Column::Center) => true,
            &Coord(Row::Top, Column::Left) | &Coord(Row::Bottom, Column::Right)
                if *diagonal == Diagonal::TopLeftToBottomRight =>
            {
                true
            }

            &Coord(Row::Bottom, Column::Left) | &Coord(Row::Top, Column::Right)
                if *diagonal == Diagonal::BottomLeftToTopRight =>
            {
                true
            }

            _ => false,
        },
        Winning::Tie => false,
    } {
        ButtonStyle::Success
    } else {
        ButtonStyle::Secondary
    }
}
