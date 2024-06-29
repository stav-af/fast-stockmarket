use fssm::market::order::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_order_market_vs_limit() {
        let market_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Market,
            details: OrderDetails { time: 1, stock: Stock::AAPL, amount: 1, lifetime_nanos: None },
        };
        let limit_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Limit { price: 100.0 },
            details: OrderDetails { time: 2, stock: Stock::AAPL , amount: 1, lifetime_nanos: None},
        };

        assert!(market_order > limit_order, "Market order should be greater than limit order");
    }

    #[test]
    fn sell_order_limit_price_priority() {
        let lower_price_order = Order {
            order_type: OrderType:: Sell,
            variant: OrderVariant::Limit { price: 95.0 },
            details: OrderDetails { time: 2, stock: Stock::GOOGL , amount: 1, lifetime_nanos: None },
        };
        let higher_price_order = Order {
            order_type: OrderType::Sell,
            variant: OrderVariant::Limit { price: 100.0 },
            details: OrderDetails { time: 1, stock: Stock::GOOGL, amount: 1, lifetime_nanos: None}
        };

        assert!(lower_price_order > higher_price_order, "Lower price sell order should have higher priority");
    }

     #[test]
    fn buy_order_limit_price_priority() {
        let lower_price_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Limit { price: 95.0 },
            details: OrderDetails { time: 2, stock: Stock::GOOGL , amount: 1, lifetime_nanos: None },
        };
        let higher_price_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Limit { price: 100.0 },
            details: OrderDetails { time: 1, stock: Stock::GOOGL, amount: 1, lifetime_nanos: None }
        };

        assert!(lower_price_order < higher_price_order, "Higher price buy order should have higher priority");
    } 

    #[test]
    fn buy_order_limit_orders_equal_prices_compare_times() {
        let earlier_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Limit { price: 150.0 },
            details: OrderDetails { time: 1, stock: Stock::MSFT , amount: 1, lifetime_nanos: None},
        };
        let later_order = Order {
            order_type: OrderType::Buy,
            variant: OrderVariant::Limit { price: 150.0 },
            details: OrderDetails { time: 2, stock: Stock::MSFT , amount: 1, lifetime_nanos: None},
        };

        assert!(earlier_order > later_order, "Earlier limit buy order should be less than later one with the same price");
    }

    #[test]
    fn sell_order_market_orders_compare_times() {
        let earlier_order = Order {
            order_type: OrderType::Sell,
            variant: OrderVariant::Market,
            details: OrderDetails { time: 1, stock: Stock::AAPL , amount: 1, lifetime_nanos: None},
        };
        let later_order = Order {
            order_type: OrderType::Sell,
            variant: OrderVariant::Market,
            details: OrderDetails { time: 2, stock: Stock::AAPL , amount: 1, lifetime_nanos: None},
        };

        assert!(earlier_order > later_order, "Earlier market sell order should be greater than later one");
    }
}