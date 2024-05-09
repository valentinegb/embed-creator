// Embed Creator Discord application
// Copyright (C) 2024  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// You may contact me via electronic mail at <valentinegb@icloud.com>.

use anyhow::{bail, Result};
use serenity::all::{
    AutocompleteChoice, Color, CommandInteraction, CommandOptionType, CreateAutocompleteResponse,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, InstallationContext, InteractionContext, ResolvedValue,
};

pub(super) fn register() -> CreateCommand {
    const COLOR_NAME: &str = "color";
    const COLOR_DESCRIPTION: &str = "Color of your embed";

    CreateCommand::new("embed")
        .description("Create and send an embed")
        .set_options(vec![
            CreateCommandOption::new(CommandOptionType::String, "title", "Title of your embed")
                .max_length(256),
            CreateCommandOption::new(
                CommandOptionType::String,
                "description",
                "Description of your embed",
            )
            .max_length(4096),
            CreateCommandOption::new(CommandOptionType::String, "url", "URL of your embed"),
            CreateCommandOption::new(CommandOptionType::String, COLOR_NAME, COLOR_DESCRIPTION)
                .name_localized("en-US", COLOR_NAME)
                .description_localized("en-US", COLOR_DESCRIPTION)
                .name_localized("en-GB", "colour")
                .description_localized("en-GB", "Colour of your embed")
                .set_autocomplete(true),
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "debug",
                "Whether to show debug representation of your embed instead",
            ),
        ])
        .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::PrivateChannel,
        ])
}

pub(super) fn execute(interaction: CommandInteraction) -> Result<CreateInteractionResponse> {
    let options = interaction.data.options();
    let mut embed = CreateEmbed::new();
    let mut has_title = false;
    let mut has_description = false;
    let mut is_debug = false;

    for option in options {
        match option.name {
            "title" => match option.value {
                ResolvedValue::String(value) => {
                    embed = embed.title(value);
                    has_title = true;
                }
                _ => bail!("Expected value of option `title` to be a string"),
            },
            "description" => match option.value {
                ResolvedValue::String(value) => {
                    embed = embed.description(value);
                    has_description = true;
                }
                _ => bail!("Expected value of option `description` to be a string"),
            },
            "url" => match option.value {
                ResolvedValue::String(value) => {
                    if has_title {
                        embed = embed.url(value);
                    } else {
                        bail!("Embed must have title to have URL");
                    }
                }
                _ => bail!("Expected value of option `url` to be a string"),
            },
            "color" => match option.value {
                ResolvedValue::String(value) => match value {
                    "BLITZ_BLUE" | "Blitz Blue" => embed = embed.color(Color::BLITZ_BLUE),
                    "BLUE" | "Blue" => embed = embed.color(Color::BLUE),
                    "BLURPLE" | "Blurple" => embed = embed.color(Color::BLURPLE),
                    "DARK_BLUE" | "Dark Blue" => embed = embed.color(Color::DARK_BLUE),
                    "DARK_GOLD" | "Dark Gold" => embed = embed.color(Color::DARK_GOLD),
                    "DARK_GREEN" | "Dark Green" => embed = embed.color(Color::DARK_GREEN),
                    "DARK_GREY" | "Dark Grey" => embed = embed.color(Color::DARK_GREY),
                    "DARK_MAGENTA" | "Dark Magenta" => embed = embed.color(Color::DARK_MAGENTA),
                    "DARK_ORANGE" | "Dark Orange" => embed = embed.color(Color::DARK_ORANGE),
                    "DARK_PURPLE" | "Dark Purple" => embed = embed.color(Color::DARK_PURPLE),
                    "DARK_RED" | "Dark Red" => embed = embed.color(Color::DARK_RED),
                    "DARK_TEAL" | "Dark Teal" => embed = embed.color(Color::DARK_TEAL),
                    "DARKER_GREY" | "Darker Grey" => embed = embed.color(Color::DARKER_GREY),
                    "FABLED_PINK" | "Fabled Pink" => embed = embed.color(Color::FABLED_PINK),
                    "FADED_PURPLE" | "Faded Purple" => embed = embed.color(Color::FADED_PURPLE),
                    "FOOYOO" | "Fooyoo" => embed = embed.color(Color::FOOYOO),
                    "GOLD" | "Gold" => embed = embed.color(Color::GOLD),
                    "KERBAL" | "Kerbal" => embed = embed.color(Color::KERBAL),
                    "LIGHT_GREY" | "Light Grey" => embed = embed.color(Color::LIGHT_GREY),
                    "LIGHTER_GREY" | "Lighter Grey" => embed = embed.color(Color::LIGHTER_GREY),
                    "MAGENTA" | "Magenta" => embed = embed.color(Color::MAGENTA),
                    "MEIBE_PINK" | "Meibe Pink" => embed = embed.color(Color::MEIBE_PINK),
                    "ORANGE" | "Orange" => embed = embed.color(Color::ORANGE),
                    "PURPLE" | "Purple" => embed = embed.color(Color::PURPLE),
                    "RED" | "Red" => embed = embed.color(Color::RED),
                    "ROHRKATZE_BLUE" | "Rohrkatze Blue" => {
                        embed = embed.color(Color::ROHRKATZE_BLUE)
                    }
                    "ROSEWATER" | "Rosewater" => embed = embed.color(Color::ROSEWATER),
                    "TEAL" | "Teal" => embed = embed.color(Color::TEAL),
                    other => bail!("Got an unexpected color: {other}"),
                },
                _ => {
                    let color_word = if interaction.locale == "en-GB" {
                        "colour"
                    } else {
                        "color"
                    };

                    bail!("Expected value of option `{color_word}` to be a string");
                }
            },
            "debug" => match option.value {
                ResolvedValue::Boolean(debug) => is_debug = debug,
                _ => bail!("Expected value of option `debug` to be a boolean"),
            },
            other => bail!("Received unknown or unimplemented option `{other}`"),
        }
    }

    if !has_title && !has_description {
        bail!("Embed must have at least a title or description");
    }

    match is_debug {
        true => Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("```rs\n{embed:#?}\n```")),
        )),
        false => Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(embed),
        )),
    }
}

pub(super) fn autocomplete(interaction: CommandInteraction) -> CreateInteractionResponse {
    let choices = vec![
        ("Blitz Blue", "BLITZ_BLUE"),
        ("Blue", "BLUE"),
        ("Blurple", "BLURPLE"),
        ("Dark Blue", "DARK_BLUE"),
        ("Dark Gold", "DARK_GOLD"),
        ("Dark Green", "DARK_GREEN"),
        ("Dark Grey", "DARK_GREY"),
        ("Dark Magenta", "DARK_MAGENTA"),
        ("Dark Orange", "DARK_ORANGE"),
        ("Dark Purple", "DARK_PURPLE"),
        ("Dark Red", "DARK_RED"),
        ("Dark Teal", "DARK_TEAL"),
        ("Darker Grey", "DARKER_GREY"),
        ("Fabled Pink", "FABLED_PINK"),
        ("Faded Purple", "FADED_PURPLE"),
        ("Fooyoo", "FOOYOO"),
        ("Gold", "GOLD"),
        ("Kerbal", "KERBAL"),
        ("Light Grey", "LIGHT_GREY"),
        ("Lighter Grey", "LIGHTER_GREY"),
        ("Magenta", "MAGENTA"),
        ("Meibe Pink", "MEIBE_PINK"),
        ("Orange", "ORANGE"),
        ("Purple", "PURPLE"),
        ("Red", "RED"),
        ("Rohrkatze Blue", "ROHRKATZE_BLUE"),
        ("Rosewater", "ROSEWATER"),
        ("Teal", "TEAL"),
    ];
    let mut filtered_choices: Vec<AutocompleteChoice> = choices
        .iter()
        .filter_map(|(name, value)| {
            if name.to_lowercase().contains(
                &interaction
                    .data
                    .autocomplete()
                    .unwrap()
                    .value
                    .to_lowercase(),
            ) {
                Some(AutocompleteChoice::new(*name, *value))
            } else {
                None
            }
        })
        .collect();

    filtered_choices.truncate(25);

    CreateInteractionResponse::Autocomplete(
        CreateAutocompleteResponse::new().set_choices(filtered_choices),
    )
}
