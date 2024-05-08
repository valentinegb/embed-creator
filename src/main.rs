use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::{
    async_trait, body,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use serenity::{
    all::{
        Command, Context, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, InstallationContext,
        Interaction, InteractionContext, Ready, ShardManager, Verifier,
    },
    prelude::TypeMapKey,
    Client,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::{SecretStore, Secrets};
use tracing::info;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        info!("Setting global commands");
        Command::set_global_commands(
            &ctx,
            vec![CreateCommand::new("embed")
                .description("Create and send an embed")
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::Guild,
                    InteractionContext::PrivateChannel,
                ])],
        )
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
    info!("Parsing body");

    let Json(interaction): Json<Interaction> =
        Json::from_bytes(&body).map_err(|_err| StatusCode::BAD_REQUEST)?;

    info!("Responding");

    match interaction {
        Interaction::Ping(_) => Ok(Json(CreateInteractionResponse::Pong)),
        Interaction::Command(_interaction) => Ok(Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("I'm aliiiiive!")
                .ephemeral(true),
        ))),
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
