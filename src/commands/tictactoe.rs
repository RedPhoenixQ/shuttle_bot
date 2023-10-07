use std::collections::HashMap;
use std::fmt::Display;

use super::*;

#[derive(Debug, Default)]
pub struct TicTacToe {
    state: HashMap<Coord, Tile>,
    winning: Option<Winning>,
}

impl TicTacToe {
    fn new(state: HashMap<Coord, Tile>) -> Self {
        let winning = calculate_winner(&state);
        Self { state, winning }
    }
}

#[async_trait]
impl CustomCommand for TicTacToe {
    const NAME: &'static str = "TicTacToe";
    fn command() -> CreateApplicationCommand {
        CreateApplicationCommand::default()
            .kind(command::CommandType::User)
            .name(Self::NAME)
            .to_owned()
    }

    async fn slash(
        ctx: Context,
        command: application_command::ApplicationCommandInteraction,
    ) -> Result<()> {
        if let Some(application_command::ResolvedTarget::User(target, _)) = command.data.target() {
            if target.bot {
                command
                    .create_interaction_response(&ctx, |data| {
                        data.interaction_response_data(|res| {
                            res.content("You cannot challenge a bot to TicTacToe!")
                                .ephemeral(true)
                        })
                    })
                    .await?;
            } else {
                command
                    .create_interaction_response(&ctx, |data| {
                        data.interaction_response_data(|res| {
                            res.content(
                                MessageBuilder::default()
                                    .mention(&target)
                                    .push_line(" has been challenged to TicTacToe!")
                                    .push(X_EMOJI)
                                    .mention(&target)
                                    .push("'s turn"),
                            )
                            .components(|c| create_components(c, &TicTacToe::default()))
                        })
                    })
                    .await?;
            }
            Ok(())
        } else {
            Err(anyhow!("No user for the user command tictactoe"))
        }
    }

    async fn component(
        ctx: Context,
        interaction: message_component::MessageComponentInteraction,
    ) -> Result<()> {
        let is_challenger =
            interaction.user.id == interaction.message.interaction.as_ref().unwrap().user.id;

        // Check is challenger because challeger may not be part of the mentions
        if !is_challenger
            && !interaction
                .message
                .mentions
                .iter()
                .any(|user| user.id == interaction.user.id)
        {
            interaction
                .create_interaction_response(&ctx, |res| {
                    res.interaction_response_data(|data| {
                        data.content("You are not part of this game")
                            .ephemeral(true)
                    })
                })
                .await?;
            return Ok(());
        }

        let clicked_coord: Coord = interaction
            .data
            .custom_id
            .as_str()
            .split_once("_")
            .unwrap()
            .1
            .try_into()
            .unwrap();

        let game = TicTacToe::new(
            interaction
                .message
                .components
                .iter()
                .flat_map(|row| {
                    row.components.iter().map(|component| {
                        if let component::ActionRowComponent::Button(button) = component {
                            let coord = button
                                .custom_id
                                .as_ref()
                                .unwrap()
                                .split_once("_")
                                .unwrap()
                                .1
                                .try_into()
                                .unwrap();
                            let tile = match &button.emoji {
                                Some(e) if e.unicode_eq(X_EMOJI) => Tile::X,
                                Some(e) if e.unicode_eq(O_EMOJI) => Tile::O,
                                _ if clicked_coord == coord => {
                                    if is_challenger {
                                        Tile::O
                                    } else {
                                        Tile::X
                                    }
                                }
                                _ => Tile::Empty,
                            };
                            (coord, tile)
                        } else {
                            unreachable!();
                        }
                    })
                })
                .collect(),
        );

        // If you are the challenger and its your turn, the mentions should include you AND your opponent
        // If you are NOT the challenger, the mentions should include ONLY you
        if is_challenger == (interaction.message.mentions.len() > 1) {
            interaction
                .create_interaction_response(&ctx, |res| {
                    res.kind(InteractionResponseType::UpdateMessage)
                        .interaction_response_data(|data| {
                            data.components(|c| create_components(c, &game)).content(
                                MessageBuilder::default()
                                    .push_line(
                                        interaction.message.content.split_once("\n").unwrap().0,
                                    )
                                    .push(if !is_challenger { X_EMOJI } else { O_EMOJI })
                                    .mention(if game.winning.is_some() {
                                        &interaction.user
                                    } else if is_challenger {
                                        interaction
                                            .message
                                            .mentions
                                            .iter()
                                            .find(|u| u.id != interaction.user.id)
                                            .unwrap()
                                    } else {
                                        &interaction.message.interaction.as_ref().unwrap().user
                                    })
                                    .push(if game.winning.is_some() {
                                        " is the winner!"
                                    } else {
                                        "'s turn"
                                    }),
                            )
                        })
                })
                .await?;
        } else {
            interaction
                .create_interaction_response(&ctx, |res| {
                    res.interaction_response_data(|data| {
                        data.content("Its not your turn").ephemeral(true)
                    })
                })
                .await?;
        };
        Ok(())
    }
}

const COMPONENT_ROWS: [Row; 3] = [Row::Bottom, Row::Middle, Row::Top];
const COMPONENT_COLUMNS: [Column; 3] = [Column::Left, Column::Center, Column::Right];

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

fn create_components<'a, 'b>(
    components: &'a mut CreateComponents,
    game: &'b TicTacToe,
) -> &'a mut CreateComponents {
    COMPONENT_ROWS.into_iter().for_each(|row| {
        components.create_action_row(|action_row| {
            COMPONENT_COLUMNS.into_iter().for_each(|col| {
                action_row.create_button(|button| {
                    let coord = Coord(row, col);
                    let tile = game.state.get(&coord).unwrap_or(&Tile::Empty);
                    button
                        .disabled(*tile != Tile::Empty || game.winning.is_some())
                        .style(match &game.winning {
                            Some(value) => get_style(&coord, value),
                            None => component::ButtonStyle::Secondary,
                        })
                        .custom_id(&coord)
                        .emoji(ReactionType::Unicode(tile.to_string()))
                });
            });
            action_row
        });
    });
    components
}

fn get_style(id: &Coord, value: &Winning) -> component::ButtonStyle {
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
    } {
        component::ButtonStyle::Success
    } else {
        component::ButtonStyle::Secondary
    }
}
