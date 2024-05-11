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

use anyhow::Context as _;
use poise::{
    command,
    serenity_prelude::{ClientBuilder, CreateEmbed, GatewayIntents},
    CreateReply, Modal,
};
use shuttle_runtime::{SecretStore, Secrets};
use shuttle_serenity::ShuttleSerenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type ApplicationContext<'a> = poise::ApplicationContext<'a, UserData, Error>;

/// User data, which is stored and accessible in all command invocations
struct UserData {}

#[derive(Modal)]
struct EmbedWizardModal {
    title: String,
    #[paragraph]
    description: String,
}

/// Create an embed, with some magic
#[command(slash_command)]
async fn embed_wizard(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let EmbedWizardModal { title, description } = EmbedWizardModal::execute(ctx)
        .await?
        .context("Ran out of time for modal to submit")?;

    ctx.send(
        CreateReply::default().embed(CreateEmbed::new().title(title).description(description)),
    )
    .await?;

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
