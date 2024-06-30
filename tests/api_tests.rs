use fssm::classes::api::{request_classes::*, response_classes::*};

use fssm::handlers::api_handler::*;
use fssm::classes::shared::order::OrderType::*;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App};
    use serde_json::json;

    // Mocks or setup functions for your dependencies
    // For example, mocking the stockmap or the buy/sell functions

    #[actix_rt::test]
    async fn test_handle_buy_order_market_price() {
        // Assuming stockmap is accessible and properly initialized
        let order_dto = OrderDTO {
            stock_name: "MSFT".to_string(),
            amount: 10,
            price: None, // Market price
        };

        let ipo_dto = IpoDTO {
            stock_name: "MSFT".to_string(),
            amount: 10,
            price: 10.0, // Market price           
        };
        let req_ipo = test::TestRequest::default()
            .set_json(&ipo_dto)
            .to_http_request();
        let payload_ipo = web::Json(ipo_dto);

        
        let req = test::TestRequest::default()
            .set_json(&order_dto)
            .to_http_request();
        let payload = web::Json(order_dto);

        let response = handle_ipo(payload_ipo);
        let resp = handle_order(payload, Sell).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        // Further assertions based on the expected behavior of buy_market
    }
}

