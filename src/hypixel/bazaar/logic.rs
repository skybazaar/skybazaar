use crate::hypixel::bazaar::models::{BazaarProduct, Price};

impl BazaarProduct {
    pub fn quick_buy_price(&self) -> Price {
        self.quick_status.buy_price
    }

    pub fn top_buy_order(&self) -> Option<Price> {
        self.buy_summary.first().map(|order| order.price_per_unit)
    }

    pub fn top_sell_order(&self) -> Option<Price> {
        self.sell_summary.first().map(|order| order.price_per_unit)
    }

    pub fn quick_spread(&self) -> Price {
        (self.quick_status.buy_price - self.quick_status.sell_price).abs()
    }

    pub fn quick_spread_percentage(&self) -> Price {
        if self.quick_status.sell_price == 0.0 {
            return 0.0;
        };
        (self.quick_spread() / self.quick_status.sell_price) * 100.0
    }

    pub fn spread(&self) -> Option<Price> {
        let top_buy = self.top_buy_order()?;
        let top_sell = self.top_sell_order()?;

        Some((top_buy - top_sell).abs())
    }

    pub fn spread_percentage(&self) -> Option<Price> {
        let cost = self.top_sell_order()?;
        if cost <= 0.0 {
            return None;
        };

        Some((self.spread()? / cost) * 100.0)
    }

    pub fn real_flip_profit(&self, tax_rate: f64) -> Option<Price> {
        let revenue = self.top_buy_order()?;
        let cost = self.top_sell_order()?;

        let tax = cost * tax_rate;

        Some(revenue - cost - tax)
    }
}
