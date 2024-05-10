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

mod commands;

use std::{io::Read, sync::Arc};

use anyhow::{anyhow, Result};
use axum::{
    async_trait, body,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use serenity::{
    all::{
        Color, Command, Context, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, Interaction, Ready,
        ShardManager, Verifier,
    },
    prelude::TypeMapKey,
    Client,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::{SecretStore, Secrets};
use tracing::{debug, error, info};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        info!("Setting global commands");
        Command::set_global_commands(&ctx, vec![commands::embed::register()])
            .await
            .unwrap();
        info!("Shutting down Discord client");

        let data = ctx.data.read().await;

        data.get::<ShardManagerContainer>()
            .unwrap()
            .shutdown_all()
            .await;
    }
}

async fn interactions(
    headers: HeaderMap,
    body: body::Bytes,
) -> Result<Json<CreateInteractionResponse>, StatusCode> {
    info!("Received interaction, verifying security");

    let signature = headers
        .get("X-Signature-Ed25519")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let verifier =
        Verifier::new("4a8dd5ca71782f7e31b7140b01ec976be6b6a21311b5eca930deb67c98be20f0");

    verifier
        .verify(
            signature
                .to_str()
                .map_err(|_err| StatusCode::UNAUTHORIZED)?,
            timestamp
                .to_str()
                .map_err(|_err| StatusCode::UNAUTHORIZED)?,
            &body,
        )
        .map_err(|_err| StatusCode::UNAUTHORIZED)?;
    info!("Security verified, parsing body");

    let Json(interaction): Json<Interaction> = Json::from_bytes(&body).map_err(|_err| {
        error!("Failed to parse body");

        let mut string = String::new();

        if let Ok(_) = body.as_ref().read_to_string(&mut string) {
            debug!("As a string, it was: {string}");
        }

        StatusCode::BAD_REQUEST
    })?;

    debug!("{interaction:#?}");
    info!("Responding to interaction");

    match interaction {
        Interaction::Ping(_) => Ok(Json(CreateInteractionResponse::Pong)),
        Interaction::Command(interaction) => Ok(Json(
            commands::embed::execute(interaction).unwrap_or_else(|err| {
                error!("Failed to run embed command: {err}");

                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(
                            CreateEmbed::new()
                                .title("Error")
                                .description(err.to_string())
                                .color(Color::RED),
                        )
                        .ephemeral(true),
                )
            }),
        )),
        Interaction::Autocomplete(interaction) => {
            Ok(Json(commands::embed::autocomplete(interaction)))
        }
        _ => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

#[shuttle_runtime::main]
async fn main(#[Secrets] secrets: SecretStore) -> ShuttleAxum {
    info!("Building Discord client");

    let mut client = Client::builder(
        secrets
            .get("DISCORD_TOKEN")
            .ok_or(anyhow!("Expected DISCORD_TOKEN secret to exist"))?,
        GatewayIntents::empty(),
    )
    .event_handler(Handler)
    .await
    .map_err(|err| anyhow!(err))?;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    client.start().await.map_err(|err| anyhow!(err))?;
    info!("Creating router");

    let router = Router::new().route("/", post(interactions));

    Ok(router.into())
}
