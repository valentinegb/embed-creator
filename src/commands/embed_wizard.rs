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

use anyhow::{anyhow, bail, Result};
use serenity::all::{
    ActionRowComponent, CreateActionRow, CreateCommand, CreateEmbed, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, InputTextStyle,
    InstallationContext, InteractionContext, ModalInteraction,
};

pub(crate) const NAME: &str = "embed_wizard";

pub(crate) fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Create an embed step-by-step with your hand held along the way")
        .integration_types(vec![InstallationContext::Guild, InstallationContext::User])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::PrivateChannel,
        ])
}

pub(crate) fn execute() -> Result<CreateInteractionResponse> {
    Ok(CreateInteractionResponse::Modal(
        CreateModal::new(format!("{NAME}:title_and_description"), "Embed Wizard").components(vec![
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Title", "title")
                    .max_length(256)
                    .required(false),
            ),
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Paragraph, "Description", "description")
                    .required(false),
            ),
        ]),
    ))
}

pub(crate) fn modal_submit(
    interaction: ModalInteraction,
    custom_id: &str,
) -> Result<CreateInteractionResponse> {
    match custom_id {
        "title_and_description" => {
            let mut title = String::new();
            let mut description = String::new();

            for row in interaction.data.components {
                for component in row.components {
                    if let ActionRowComponent::InputText(input_text) = component {
                        match input_text.custom_id.as_str() {
                            "title" => {
                                title = input_text
                                    .value
                                    .ok_or(anyhow!("`title` component is missing"))?
                            }
                            "description" => {
                                description = input_text
                                    .value
                                    .ok_or(anyhow!("`description` component is missing"))?
                            }
                            _ => (),
                        }
                    }
                }
            }

            if title.is_empty() && description.is_empty() {
                bail!("Embed must have at least a title or description");
            }

            Ok(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(CreateEmbed::new().title(title).description(description)),
            ))
        }
        other => bail!("Unknown modal `{other}`"),
    }
}
