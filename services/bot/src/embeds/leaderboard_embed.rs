use poise::serenity_prelude::{CreateEmbed, PartialGuild, UserId};

use bot_sdk::models::UserRanking;

use crate::commands::{Context, leaderboard::WhoseScores};

pub async fn leaderboard_embed(
    ctx: &Context<'_>,
    guild: &PartialGuild,
    whose_scores: &WhoseScores,
    rankings: Vec<UserRanking>,
    highlight_user_id: Option<&str>,
) -> CreateEmbed {
    let names_and_rankings = rankings
        .into_iter()
        .map(|r| resolve_name_with_ranking(ctx, guild, r));
    let names_and_rankings = futures::future::join_all(names_and_rankings).await;

    let lines: Vec<String> = names_and_rankings
        .iter()
        .map(|(name, r)| {
            let name: &str = match name {
                Some(n) => n,
                None => "N/A",
            };

            let line = format!("{}. {}: {}", r.ranking, name, r.correct);
            let line = match highlight_user_id {
                Some(h) => {
                    if h == r.user_id {
                        format!("**{line}**")
                    } else {
                        line
                    }
                }
                None => line,
            };

            line
        })
        .collect();

    let lines_concat = lines.join("\n");

    let title = match whose_scores {
        WhoseScores::Top10 => "Name That Card: Top 10",
        WhoseScores::Me => "Name That Card: Your Score",
    };

    let embed = CreateEmbed::default()
        .title(title)
        .description(lines_concat);

    embed
}

async fn resolve_name_with_ranking(
    ctx: &Context<'_>,
    guild: &PartialGuild,
    ranking: UserRanking,
) -> (Option<String>, UserRanking) {
    let user_id = ranking.user_id.parse::<u64>().map(|u| UserId::from(u));

    let user_id = match user_id {
        Ok(u) => u,
        Err(_) => return (None, ranking),
    };

    let user = user_id.to_user(ctx).await;
    let user = match user {
        Ok(u) => u,
        Err(_) => return (None, ranking),
    };

    let nick_name = user.nick_in(ctx, guild).await;
    let name = &user.name;

    let name = match nick_name {
        Some(nick) => nick,
        None => match name.as_ref() {
            "" => "??".to_string(),
            _ => name.to_string(),
        },
    };

    return (Some(name), ranking);
}
