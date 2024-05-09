use anyhow::{bail, Result};
use serenity::all::{
    AutocompleteChoice, CommandInteraction, CommandOptionType, CreateAutocompleteResponse,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, InstallationContext, InteractionContext, ResolvedValue,
};

pub(super) fn register() -> CreateCommand {
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
            CreateCommandOption::new(CommandOptionType::String, "color", "Color of your embed")
                .set_autocomplete(true),
        ])
        .integration_types(vec![InstallationContext::User])
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
            other => bail!("Received unknown or unimplemented option `{other}`"),
        }
    }

    if !has_title && !has_description {
        bail!("Embed must have at least a title or description");
    }

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
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
