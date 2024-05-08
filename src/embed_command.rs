use anyhow::{bail, Result};
use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, InstallationContext,
    InteractionContext, ResolvedValue,
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
        ])
        .integration_types(vec![InstallationContext::User])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::PrivateChannel,
        ])
}

pub(super) async fn execute(interaction: CommandInteraction) -> Result<CreateInteractionResponse> {
    let options = interaction.data.options();
    let mut embed = CreateEmbed::new();

    for option in options {
        match option.name {
            "title" => match option.value {
                ResolvedValue::String(value) => {
                    embed = embed.title(value);
                }
                _ => bail!("Expected value of option `title` to be a string"),
            },
            "description" => match option.value {
                ResolvedValue::String(value) => {
                    embed = embed.description(value);
                }
                _ => bail!("Expected value of option `description` to be a string"),
            },
            other => bail!("Received unknown option `{other}`"),
        }
    }

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}
