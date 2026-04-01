use crate::hypixel::{
    bazaar::models::BazaarData,
    items::{ITEMS, Item},
};

impl BazaarData {
    pub fn all_npc_flips(&self) -> Vec<(&Item, f64)> {
        self.data
            .values()
            .into_iter()
            .filter_map(|bz_item| ITEMS.get(&bz_item.product_id).map(|item| (bz_item, item)))
            .filter_map(|(bz_item, item)| item.npc_sell_price.map(|nsp| (bz_item, item, nsp)))
            .map(|(bz_item, item, nsp)| (item, nsp - bz_item.quick_status.buy_price))
            .filter(|(_, diff)| *diff > 0.0)
            .collect()
    }
}
