use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateSelectMenuOption;
use sdk::models::Character;

pub fn did_you_mean(
    characters: Vec<Character>,
    original_query: String,
) -> (String, serenity::CreateActionRow) {
    let message = format!(
        r#"Failed to find a character named **{}**.
Did you mean one of these?"#,
        original_query
    );

    let select_menu_options: Vec<CreateSelectMenuOption> = characters
        .into_iter()
        .map(|c| {
            let combined_name = match (&c.name, &c.jp_name) {
                (Some(name), Some(jp_name)) => format!("{} ({})", name, jp_name),
                (Some(name), None) => name.to_string(),
                (None, Some(jp_name)) => jp_name.to_string(),
                _ => format!("? (char_id: {})", &c.char_id).to_string(),
            };
            let value = c.char_id.clone();

            serenity::CreateSelectMenuOption::new(combined_name, value)
        })
        .collect();
    let select_menu = serenity::CreateSelectMenu::new(
        "did_you_mean:",
        serenity::CreateSelectMenuKind::String {
            options: select_menu_options,
        },
    )
    .placeholder("Select character:");

    (message, serenity::CreateActionRow::SelectMenu(select_menu))
}
