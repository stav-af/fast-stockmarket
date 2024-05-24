use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Stock {
    AAPL,
    GOOGL,
    MSFT
}

#[derive(Debug, PartialEq)]
pub enum OrderVariant {
    Market,
    Limit { price: f64 }
}

#[derive(Debug)]
pub struct OrderDetails {
    pub time: f64,
    pub stock: Stock,
    pub amount: u64
}

#[derive(Debug)]
pub struct SellOrder {
    pub variant: OrderVariant,
    pub details: OrderDetails
}

#[derive(Debug)]
pub struct BuyOrder {
    pub variant: OrderVariant,
    pub details: OrderDetails
}




impl PartialOrd for BuyOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use OrderVariant::*;
        match (&self.variant, &other.variant) {
            (Market, Market) => self.details.time.partial_cmp(&other.details.time),
            (Limit { price: price1 }, Limit { price: price2 }) => {
                // First compare by price, then by time if prices are equal
                match price1.partial_cmp(price2) {
                    Some(Ordering::Equal) => self.details.time.partial_cmp(&other.details.time),
                    other => other,
                }
            },
            (Market, Limit { .. }) => Some(Ordering::Greater),
            (Limit { .. }, Market) => Some(Ordering::Less),
        }
    }
}

impl PartialEq for BuyOrder {
    fn eq(&self, other: &Self) -> bool {
        match (&self.variant, &other.variant) {
            (OrderVariant::Market, OrderVariant::Market) => self.details.time == other.details.time,
            (OrderVariant::Limit { price: price1 }, OrderVariant::Limit { price: price2 }) => price1 == price2 && self.details.time == other.details.time,
            _ => false,
        }
    }
}

impl PartialOrd for SellOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use OrderVariant::*;
        match (&self.variant, &other.variant) {
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

impl PartialEq for SellOrder {
    fn eq(&self, other: &Self) -> bool {
        match (&self.variant, &other.variant) {
            (OrderVariant::Market, OrderVariant::Market) => self.details.time == other.details.time,
            (OrderVariant::Limit { price: price1 }, OrderVariant::Limit { price: price2 }) 
                => price1 == price2 && self.details.time == other.details.time,
            _ => false,
        }
    }
}

impl Eq for SellOrder {}

impl Eq for BuyOrder {}

impl Ord for BuyOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Ord for SellOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}