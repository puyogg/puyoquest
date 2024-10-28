use phf::phf_map;

static EMOJI_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    "[Red Puyo]" => "<:red:429944006135382017>",
    "[Green Puyo]" => "<:green:429944006948945931>",
    "[Blue Puyo]" => "<:blue:429944006601080849>",
    "[Yellow Puyo]" => "<:yellow:429944006718521345>",
    "[Purple Puyo]" => "<:purple:429944007397736448>",
    "[Prism Ball]" => "<:prism:714920278404104292>",
    "[Red Chance Puyo]" => "<a:red_chance:585673630713774080>",
    "[Blue Chance Puyo]" => "<a:blue_chance:585673688003641345>",
    "[Green Chance Puyo]" => "<a:green_chance:585673527529570304>",
    "[Yellow Chance Puyo]" => "<a:yellow_chance:585673736665956372>",
    "[Purple Chance Puyo]" => "<a:purple_chance:585673780593033226>",
};

pub fn wiki_symbols_to_emojis(input_str: &str) -> String {
    let mut input = String::from(input_str);

    for (wiki_symbol, emoji_markdown) in EMOJI_TABLE.entries() {
        input = input.replace(wiki_symbol, &emoji_markdown);
    }

    input
}
