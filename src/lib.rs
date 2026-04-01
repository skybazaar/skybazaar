use crate::hypixel::items;

pub mod hypixel;

#[test]
fn test_item_phf() {
    let item = items::ITEMS.get("DIAMOND_LEGGINGS").unwrap();
    assert_eq!(item.name, "Diamond Leggings");
    assert_eq!(item.npc_sell_price.unwrap(), 28f64);
}
