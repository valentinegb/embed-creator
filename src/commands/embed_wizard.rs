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

use anyhow::Result;
use serenity::all::{
    CommandInteraction, CreateActionRow, CreateCommand, CreateInputText, CreateInteractionResponse,
    CreateModal, InputTextStyle, InstallationContext, InteractionContext,
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

pub(crate) fn execute(interaction: CommandInteraction) -> Result<CreateInteractionResponse> {
    Ok(CreateInteractionResponse::Modal(
        CreateModal::new("embed_wizard", "Embed Wizard").components(vec![
            CreateActionRow::InputText(CreateInputText::new(
                InputTextStyle::Short,
                "Title",
                "embed_wizard_title",
            )),
            CreateActionRow::InputText(CreateInputText::new(
                InputTextStyle::Paragraph,
                "Description",
                "embed_wizard_description",
            )),
        ]),
    ))
}
