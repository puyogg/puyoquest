use poise::{
    serenity_prelude::{self as serenity, CreateQuickModal},
    Modal,
};
use sdk::models::{AliasCreate, CardType};

use crate::{
    commands::{Data, Error},
    interaction_router::{
        component::check_if_wiki_editor_and_error,
        interaction_helpers::{basic_component_response, basic_modal_response},
    },
};

pub enum AliasButtonAction {
    Add,
    Delete,
    None,
}

impl std::fmt::Display for AliasButtonAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliasButtonAction::None => write!(f, "none"),
            AliasButtonAction::Add => write!(f, "add"),
            AliasButtonAction::Delete => write!(f, "delete"),
        }
    }
}

impl From<&str> for AliasButtonAction {
    fn from(value: &str) -> Self {
        match value {
            "add" => AliasButtonAction::Add,
            "delete" => AliasButtonAction::Delete,
            "none" => AliasButtonAction::None,
            _ => AliasButtonAction::None,
        }
    }
}

pub async fn alias_handler(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = &interaction.data.custom_id;
    let (char_id, action) = parse_alias_embed_custom_id(custom_id);

    let basic_response = |message: &str| {
        let message = message.to_string();
        basic_component_response(ctx, interaction, message)
    };

    let char_id = match char_id {
        None => {
            basic_response("There was an error handling your alias request!").await?;

            return Ok(());
        }
        Some(c) => c,
    };

    match action {
        AliasButtonAction::None => {
            basic_response("Invalid alias action request. Try something else.").await
        }
        AliasButtonAction::Delete => {
            check_if_wiki_editor_and_error(ctx, interaction).await?;
            delete_alias_modal(ctx, data, interaction, &char_id).await
        }
        AliasButtonAction::Add => {
            check_if_wiki_editor_and_error(ctx, interaction).await?;
            add_alias_modal(ctx, data, interaction, &char_id).await
        }
    }
}

pub async fn add_alias_modal(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    char_id: &str,
) -> Result<(), Error> {
    let basic_response = |message: &str| {
        let message = message.to_string();
        basic_component_response(ctx, interaction, message)
    };

    let character =
        sdk::apis::characters_api::characters_id_get(&data.api_config, &char_id).await?;
    let modal = CreateQuickModal::new(format!(
        "Add an alias for: {}",
        &character.name.clone().unwrap_or("??".to_string())
    ))
    .timeout(std::time::Duration::from_secs(120))
    .short_field("Name");

    let modal_user_response = interaction.quick_modal(ctx, modal).await?;
    let modal_user_response = match modal_user_response {
        Some(r) => r,
        None => {
            basic_response("There was an error handling your alias request.").await?;
            return Ok(());
        }
    };

    let inputs = modal_user_response.inputs;
    let submitted_name = inputs.get(0);
    let submitted_name = match submitted_name {
        Some(name) => {
            if name.trim().len() == 0 {
                basic_response("Invalid alias! Try something else.").await?;
                return Ok(());
            }
            name
        }
        None => {
            basic_response("Invalid alias! Try something else.").await?;
            return Ok(());
        }
    };

    let upserted_alias = sdk::apis::aliases_api::aliases_post(
        &data.api_config,
        AliasCreate {
            alias: submitted_name.to_string(),
            char_id: char_id.to_string(),
            internal: false,
            card_type: match &character.link_name {
                Some(link_name) => {
                    // TODO: not really accurate
                    if link_name.to_lowercase().contains("/material") {
                        CardType::Material
                    } else {
                        CardType::Character
                    }
                }
                None => CardType::Character,
            },
            updated_at: None,
        },
    )
    .await;

    let modal_response =
        |message: String| basic_modal_response(ctx, &modal_user_response.interaction, message);
    let user_id = &modal_user_response.interaction.user.id;

    match upserted_alias {
        Ok(a) => {
            modal_response(format!(
                "<@{}> Successfully added alias **{}** for character: **{}** (id: {})",
                &user_id,
                &a.alias,
                &character.name.unwrap_or("??".to_string()),
                &character.char_id
            ))
            .await?;
        }
        Err(_e) => {
            modal_response(format!(
                "<@{}> There was a problem adding alias **{}** for character: **{}** (id: {})",
                &user_id,
                &submitted_name,
                &character.name.unwrap_or("??".to_string()),
                &character.char_id
            ))
            .await?;
        }
    }

    Ok(())
}

pub async fn delete_alias_modal(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::model::application::ComponentInteraction,
    char_id: &str,
) -> Result<(), Error> {
    let basic_response = |message: &str| {
        let message = message.to_string();
        basic_component_response(ctx, interaction, message)
    };

    let character =
        sdk::apis::characters_api::characters_id_get(&data.api_config, &char_id).await?;
    let modal = CreateQuickModal::new(format!(
        "Delete an alias for: {}",
        &character.name.clone().unwrap_or("??".to_string())
    ))
    .timeout(std::time::Duration::from_secs(120))
    .short_field("Name");

    let modal_user_response = interaction.quick_modal(ctx, modal).await?;
    let modal_user_response = match modal_user_response {
        Some(r) => r,
        None => {
            basic_response("There was an error handling your alias request.").await?;
            return Ok(());
        }
    };

    let inputs = modal_user_response.inputs;
    let submitted_name = inputs.get(0);
    let submitted_name = match submitted_name {
        Some(name) => {
            if name.trim().len() == 0 {
                basic_response("Invalid alias! Try something else.").await?;
                return Ok(());
            }
            name
        }
        None => {
            basic_response("Invalid alias! Try something else.").await?;
            return Ok(());
        }
    };

    let delete_response =
        sdk::apis::aliases_api::aliases_delete(&data.api_config, &submitted_name).await;

    let modal_response =
        |message: String| basic_modal_response(ctx, &modal_user_response.interaction, message);
    let user_id = &modal_user_response.interaction.user.id;

    match delete_response {
        Ok(_d) => {
            modal_response(format!(
                "<@{}> Successfully deleted alias **{}** for character: **{}** (id: {})",
                user_id,
                &submitted_name,
                &character.name.unwrap_or("??".to_string()),
                &character.char_id
            ))
            .await?;
        }
        Err(e) => {
            println!("{:?}", e);
            modal_response(format!(
                "<@{}> There was a problem deleting alias **{}** for character: **{}** (id: {})",
                user_id,
                &submitted_name,
                &character.name.unwrap_or("??".to_string()),
                &character.char_id
            ))
            .await?;
        }
    }

    Ok(())
}

pub fn parse_alias_embed_custom_id(custom_id: &str) -> (Option<String>, AliasButtonAction) {
    let values = custom_id.split(":").collect::<Vec<&str>>();

    let char_id = values.get(3).map(|c| c.to_string());
    let action = values
        .get(4)
        .map(|a| AliasButtonAction::from(*a))
        .unwrap_or(AliasButtonAction::None);

    (char_id, action)
}

pub fn alias_nav_button(char_id: &str, action: AliasButtonAction) -> serenity::CreateButton {
    let custom_id = format!("alias:options::{}:{}", char_id, action.to_string());

    match action {
        AliasButtonAction::None => serenity::CreateButton::new(custom_id).label("N/A"),
        AliasButtonAction::Add => serenity::CreateButton::new(custom_id).label("Alias: Add"),
        AliasButtonAction::Delete => serenity::CreateButton::new(custom_id).label("Alias: Delete"),
    }
}

#[derive(Debug, poise::Modal)]
#[name = "Add a new alias"]
struct AliasAddModal {
    #[name = "Add a new alias"]
    #[min_length = 2]
    #[max_length = 64]
    alias_input: String,
}
