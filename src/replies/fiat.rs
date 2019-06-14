use titlecase::titlecase;

pub fn get_fiat(db: &super::db::DB, coin: String, amount: f32) -> Option<String> {
    let price = db.get_latest_price(coin);
    if let Some(p) = price {
        return Some(format!("{} {} ({}) is worth €{} at €{} per coin", amount, titlecase(&p.name), p.ticker.to_uppercase(),
                            super::format_currency(amount * p.euro), super::format_currency(p.euro)))
    }

    None
}