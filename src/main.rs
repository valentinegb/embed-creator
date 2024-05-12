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

use std::fmt::{Debug, Display};

use anyhow::{bail, Context as _};
use poise::{
    command,
    serenity_prelude::{
        self, futures::StreamExt, ActionRowComponent, ButtonKind, ButtonStyle, CacheHttp,
        ClientBuilder, Color, ComponentInteraction, ComponentInteractionDataKind, ComponentType,
        CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption, GatewayIntents,
    },
    CreateReply, FrameworkError, Modal,
};
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_serenity::ShuttleSerenity;
use tracing::error;

type Error = anyhow::Error;
type ApplicationContext<'a> = poise::ApplicationContext<'a, UserData, Error>;

/// User data, which is stored and accessible in all command invocations
struct UserData {}

#[derive(Modal)]
#[name = "Embed Wizard"]
struct EmbedWizardModal {
    #[max_length = 256]
    title: Option<String>,
    #[paragraph]
    description: Option<String>,
}

/// Create an embed, with some magic
#[command(
    slash_command,
    guild_installable,
    user_installable,
    guild_usable,
    private_channel_usable
)]
async fn embed_wizard(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let EmbedWizardModal { title, description } = EmbedWizardModal::execute(ctx)
        .await?
        .context("Ran out of time for modal to submit")?;
    let mut embed = CreateEmbed::new();

    if title.is_none() && description.is_none() {
        bail!("You must have at least a title or a description");
    }

    if let Some(title) = title {
        embed = embed.title(title);
    }

    if let Some(description) = description {
        embed = embed.description(description);
    }

    let skip_button = CreateButton::new("skip_button")
        .label("Skip")
        .style(ButtonStyle::Danger);
    let initial_colors_components = vec![
        CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "color_select",
                CreateSelectMenuKind::String {
                    options: vec![
                        CreateSelectMenuOption::new("Blitz Blue", "BLITZ_BLUE"),
                        CreateSelectMenuOption::new("Blue", "BLUE"),
                        CreateSelectMenuOption::new("Blurple", "BLURPLE"),
                        CreateSelectMenuOption::new("Dark Blue", "DARK_BLUE"),
                        CreateSelectMenuOption::new("Dark Gold", "DARK_GOLD"),
                        CreateSelectMenuOption::new("Dark Green", "DARK_GREEN"),
                        CreateSelectMenuOption::new("Dark Grey", "DARK_GREY"),
                        CreateSelectMenuOption::new("Dark Magenta", "DARK_MAGENTA"),
                        CreateSelectMenuOption::new("Dark Orange", "DARK_ORANGE"),
                        CreateSelectMenuOption::new("Dark Purple", "DARK_PURPLE"),
                        CreateSelectMenuOption::new("Dark Red", "DARK_RED"),
                        CreateSelectMenuOption::new("Dark Teal", "DARK_TEAL"),
                        CreateSelectMenuOption::new("Darker Grey", "DARKER_GREY"),
                        CreateSelectMenuOption::new("Fabled Pink", "FABLED_PINK"),
                        CreateSelectMenuOption::new("Faded Purple", "FADED_PURPLE"),
                        CreateSelectMenuOption::new("Fooyoo", "FOOYOO"),
                        CreateSelectMenuOption::new("Gold", "GOLD"),
                        CreateSelectMenuOption::new("Kerbal", "KERBAL"),
                        CreateSelectMenuOption::new("Light Grey", "LIGHT_GREY"),
                        CreateSelectMenuOption::new("Lighter Grey", "LIGHTER_GREY"),
                        CreateSelectMenuOption::new("Magenta", "MAGENTA"),
                        CreateSelectMenuOption::new("Meibe Pink", "MEIBE_PINK"),
                        CreateSelectMenuOption::new("Orange", "ORANGE"),
                        CreateSelectMenuOption::new("Purple", "PURPLE"),
                        CreateSelectMenuOption::new("Red", "RED"),
                    ],
                },
            )
            .placeholder("Select a color"),
        ),
        CreateActionRow::Buttons(vec![
            CreateButton::new("more_colors_button")
                .label("More Colors")
                .style(ButtonStyle::Secondary),
            skip_button.clone(),
        ]),
    ];
    let color_prompt = ctx
        .send(
            CreateReply::default()
                .content("Would you like to select a color?")
                .components(initial_colors_components.clone())
                .ephemeral(true),
        )
        .await?;
    let mut color_prompt_interactions = color_prompt
        .message()
        .await?
        .await_component_interactions(ctx)
        .stream();

    while let Some(interaction) = color_prompt_interactions.next().await {
        match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => {
                embed = embed.color(match values[0].as_str() {
                    "BLITZ_BLUE" => Color::BLITZ_BLUE,
                    "BLUE" => Color::BLUE,
                    "BLURPLE" => Color::BLURPLE,
                    "DARK_BLUE" => Color::DARK_BLUE,
                    "DARK_GOLD" => Color::DARK_GOLD,
                    "DARK_GREEN" => Color::DARK_GREEN,
                    "DARK_GREY" => Color::DARK_GREY,
                    "DARK_MAGENTA" => Color::DARK_MAGENTA,
                    "DARK_ORANGE" => Color::DARK_ORANGE,
                    "DARK_PURPLE" => Color::DARK_PURPLE,
                    "DARK_RED" => Color::DARK_RED,
                    "DARK_TEAL" => Color::DARK_TEAL,
                    "DARKER_GREY" => Color::DARKER_GREY,
                    "FABLED_PINK" => Color::FABLED_PINK,
                    "FADED_PURPLE" => Color::FADED_PURPLE,
                    "FOOYOO" => Color::FOOYOO,
                    "GOLD" => Color::GOLD,
                    "KERBAL" => Color::KERBAL,
                    "LIGHT_GREY" => Color::LIGHT_GREY,
                    "LIGHTER_GREY" => Color::LIGHTER_GREY,
                    "MAGENTA" => Color::MAGENTA,
                    "MEIBE_PINK" => Color::MEIBE_PINK,
                    "ORANGE" => Color::ORANGE,
                    "PURPLE" => Color::PURPLE,
                    "RED" => Color::RED,
                    "ROHRKATZE_BLUE" => Color::ROHRKATZE_BLUE,
                    "ROSEWATER" => Color::ROSEWATER,
                    "TEAL" => Color::TEAL,
                    other => {
                        disable_components(ctx, &interaction).await?;
                        bail!("Received unexpected color `{other}`");
                    }
                });

                interaction
                    .create_response(
                        ctx,
                        CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                                .content("Got it, that's all that's implemented for now.")
                                .components(Vec::new()),
                        ),
                    )
                    .await?;

                break;
            }
            ComponentInteractionDataKind::Button => match interaction.data.custom_id.as_str() {
                "more_colors_button" => {
                    interaction
                        .create_response(
                            ctx,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new().components(vec![
                                    CreateActionRow::SelectMenu(
                                        CreateSelectMenu::new(
                                            "color_select",
                                            CreateSelectMenuKind::String {
                                                options: vec![
                                                    CreateSelectMenuOption::new(
                                                        "Rohrkatze Blue",
                                                        "ROHRKATZE_BLUE",
                                                    ),
                                                    CreateSelectMenuOption::new(
                                                        "Rosewater",
                                                        "ROSEWATER",
                                                    ),
                                                    CreateSelectMenuOption::new("Teal", "TEAL"),
                                                ],
                                            },
                                        )
                                        .placeholder("Select a color"),
                                    ),
                                    CreateActionRow::Buttons(vec![
                                        CreateButton::new("initial_colors_button")
                                            .label("Initial Colors")
                                            .style(ButtonStyle::Secondary),
                                        skip_button.clone(),
                                    ]),
                                ]),
                            ),
                        )
                        .await?;
                }
                "initial_colors_button" => {
                    interaction
                        .create_response(
                            ctx,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new()
                                    .components(initial_colors_components.clone()),
                            ),
                        )
                        .await?;
                }
                "skip_button" => {
                    interaction
                        .create_response(
                            ctx,
                            CreateInteractionResponse::UpdateMessage(
                                CreateInteractionResponseMessage::new()
                                    .content("Oh okay, well that's it for now.")
                                    .components(Vec::new()),
                            ),
                        )
                        .await?;

                    break;
                }
                other => {
                    disable_components(ctx, &interaction).await?;
                    bail!("Got unknown component ID `{other}`");
                }
            },
            other => {
                disable_components(ctx, &interaction).await?;
                bail!(
                    "Expected component kind to be `StringSelect` or `Button`, got:\n```rs\n{other:#?}\n```",
                );
            }
        }
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

async fn disable_components(
    cache_http: impl CacheHttp,
    interaction: &ComponentInteraction,
) -> serenity_prelude::Result<()> {
    let disabled_components = interaction
        .message
        .components
        .clone()
        .into_iter()
        .map(|row| match &row.components[0] {
            ActionRowComponent::Button(_) => CreateActionRow::Buttons(
                row.components
                    .into_iter()
                    .map(|component| match component {
                        ActionRowComponent::Button(button) => {
                            let mut disabled_button = match button.data {
                                ButtonKind::Link { url } => CreateButton::new_link(url),
                                ButtonKind::NonLink { custom_id, style } => {
                                    CreateButton::new(custom_id).style(style)
                                }
                            }
                            .disabled(true);

                            if let Some(label) = button.label {
                                disabled_button = disabled_button.label(label);
                            }

                            if let Some(emoji) = button.emoji {
                                disabled_button = disabled_button.emoji(emoji);
                            }

                            disabled_button
                        }
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            ActionRowComponent::SelectMenu(select) => {
                CreateActionRow::SelectMenu(match &select.custom_id {
                    Some(custom_id) => {
                        let mut disabled_select = CreateSelectMenu::new(
                            custom_id,
                            match select.kind {
                                ComponentType::StringSelect => CreateSelectMenuKind::String {
                                    options: vec![CreateSelectMenuOption::new(
                                        "You shouldn't be able to see this!",
                                        "UNREACHABLE",
                                    )],
                                },
                                ComponentType::UserSelect => CreateSelectMenuKind::User {
                                    default_users: None,
                                },
                                ComponentType::RoleSelect => CreateSelectMenuKind::Role {
                                    default_roles: None,
                                },
                                ComponentType::MentionableSelect => {
                                    CreateSelectMenuKind::Mentionable {
                                        default_users: None,
                                        default_roles: None,
                                    }
                                }
                                ComponentType::ChannelSelect => CreateSelectMenuKind::Channel {
                                    channel_types: None,
                                    default_channels: None,
                                },
                                _ => unreachable!(),
                            },
                        )
                        .disabled(true);

                        if let Some(placeholder) = &select.placeholder {
                            disabled_select = disabled_select.placeholder(placeholder);
                        }

                        disabled_select
                    }
                    None => unreachable!(),
                })
            }
            _ => unreachable!(),
        })
        .collect();

    interaction
        .create_response(
            cache_http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().components(disabled_components),
            ),
        )
        .await
}

async fn on_error<U, E: Display + Debug>(
    error: FrameworkError<'_, U, E>,
) -> Result<(), serenity_prelude::Error> {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            error!("An error occured in a command: {error}");
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title("Error")
                            .description(error.to_string())
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::CommandPanic {
            payload: _, ctx, ..
        } => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title("Fatal Error")
                            .description("A *fatal* error occured. We can't show you the details, because it might contain sensitive information, but it has been logged for the developers to look at.")
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        _ => poise::builtins::on_error(error).await?,
    }

    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("Discord token was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![embed_wizard()],
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = on_error(error).await {
                        error!("Error while handling error: {}", e);
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(UserData {})
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
