use serde::Deserialize;
use std::{fs::File, io::BufWriter, io::Write};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    // https://wiki.hypixel.net/Rarity
    // This has been changed to Divine, however the API still returns SUPREME instead
    #[serde(alias = "SUPREME")]
    Divine,
    Special,
    VerySpecial,
    Ultimate,
    Unobtainable,
}

#[derive(Deserialize)]
struct Item {
    id: String,
    name: String,
    tier: Option<Rarity>,
    npc_sell_price: Option<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = File::open("data/items.json").expect("Cannot open data/items.json for codegen.");
    let items: serde_json::Value = serde_json::from_reader(items)?;
    let items = items
        .get("items")
        .ok_or("Missing 'items' field in data/items.json")?;

    let items: Vec<Item> = serde_json::from_value(items.clone())?;

    let file = File::create("src/hypixel/items.rs")?;
    let mut file = BufWriter::new(file);

    writeln!(file, "// File generated from build.rs - DO NOT EDIT")?;
    writeln!(file, "use crate::hypixel::items::Rarity::*;")?;

    writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(file, "pub enum Rarity {{")?;
    #[rustfmt::skip]
    writeln!(file, "    Common, Uncommon, Rare, Epic, Legendary, Mythic, Divine,")?;
    writeln!(file, "    Special, VerySpecial, Ultimate, Unobtainable,")?;
    writeln!(file, "}}\n")?;

    writeln!(file, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(file, "pub struct Item {{")?;
    writeln!(file, "    pub id: &'static str,")?;
    writeln!(file, "    pub name: &'static str,")?;
    writeln!(file, "    pub tier: Option<Rarity>,")?;
    writeln!(file, "    pub npc_sell_price: Option<f64>,")?;
    writeln!(file, "}}\n")?;

    write!(file, "pub static ITEMS: phf::Map<&'static str, Item> = ")?;

    let mut builder = phf_codegen::Map::new();
    for item in items {
        let value_code = format!(
            "Item {{ id: {:?}, name: {:?}, tier: {:?}, npc_sell_price: {:?} }}",
            item.id, item.name, item.tier, item.npc_sell_price
        );

        builder.entry(item.id, value_code);
    }

    write!(file, "{}", builder.build())?;
    writeln!(file, ";")?;

    println!("cargo:rerun-if-changed=data/items.json");
    Ok(())
}
