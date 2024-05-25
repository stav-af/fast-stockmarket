use std::cmp::Ordering;

use actix_web::http::header::ByteRangeSpec;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Stock {
    AAPL,
    GOOGL,
    MSFT
}


#[derive(Debug, PartialEq)]
pub enum OrderType {
    Buy,
    Sell
}

#[derive(Debug, PartialEq)]
pub enum OrderVariant {
    Market,
    Limit { price: f64 }
}

#[derive(Debug)]
pub struct OrderDetails {
    pub time: i64,
    pub stock: Stock,
    pub amount: u64,
    pub lifetime: Option<i64>
}

#[derive(Debug)]
pub struct Order {
    pub order_type: OrderType,
    pub variant: OrderVariant,
    pub details: OrderDetails
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use OrderVariant::*;
        use OrderType::*;
        match &self.order_type {
            Buy => match (&self.variant, &other.variant) {
                (Market, Market) => self.details.time.partial_cmp(&other.details.time),
                (Limit { price: price1 }, Limit { price: price2 }) => {
                    // First compare by price, then by time if prices are equal
                    match price1.partial_cmp(price2) {
                        Some(Ordering::Equal) => other.details.time.partial_cmp(&self.details.time),
                        other => other,
                    }
                },
                (Market, Limit { .. }) => Some(Ordering::Greater),
                (Limit { .. }, Market) => Some(Ordering::Less),
            }
            Sell => match (&self.variant, &other.variant) {
                (Market, Market) => other.details.time.partial_cmp(&self.details.time),
                (Limit { price: price1 }, Limit { price: price2 }) => {
                    // Reverse price comparison: lower price has higher priority
                    match price2.partial_cmp(price1) {
                        Some(Ordering::Equal) => other.details.time.partial_cmp(&self.details.time),
                        other => other
                    }
                },
                (Market, Limit { .. }) => Some(Ordering::Greater),
                (Limit { .. }, Market) => Some(Ordering::Less),
            }
        }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        use OrderType::*;
        match &self.order_type {
            Buy => match (&self.variant, &other.variant) {
                (OrderVariant::Market, OrderVariant::Market) => self.details.time == other.details.time,
                (OrderVariant::Limit { price: price1 }, OrderVariant::Limit { price: price2 }) => price1 == price2 && self.details.time == other.details.time,
                _ => false,
            }
            Sell => match (&self.variant, &other.variant) {
                (OrderVariant::Market, OrderVariant::Market) => self.details.time == other.details.time,
                (OrderVariant::Limit { price: price1 }, OrderVariant::Limit { price: price2 }) 
                    => price1 == price2 && self.details.time == other.details.time,
                _ => false,
            }
        }
        
    }
}

impl Eq for Order {}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
