use FSSM::api_handler::{request_classes::*, handler::*};
use FSSM::market::order::OrderType::*;
use FSSM::market::market::ipo;
use FSSM::market::order::Stock::*;
use FSSM::market::order::Stock;

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{test, web, http::{self}, App, HttpResponse};
    use serde_json::json;
    #[actix_rt::test]
    async fn test_handle_buy_order_market_price() {
        // Assuming stockmap is accessible and properly initialized
        let order_dto = OrderDTO {
            stock: MSFT,
            amount: 10,
            price: None, // Market price
        };

        let ipo_dto = IpoDTO {
            stock: MSFT,
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

    #[actix_rt::test]
    async fn test_happypath_price_query() {
        ipo(MSFT, 100, 10.0);
        let query = StockQuery {
            stock: MSFT
        };
        let req = web::Query(query);

        let res = handle_price(req).expect("Threw on price query");
        assert!(res.status() == 200, "expected status 200 but found {:?}", res.status())
    } 

    #[actix_rt::test]
    async fn test_happypath_historic_price_query() {
        ipo(MSFT, 100, 10.0);

        let query = StockQuery {
            stock: MSFT
        };
        let req = web::Query(query);

        let res = handle_price(req).expect("Threw on price query");
        assert!(res.status() == 200, "expected status 200 but found {:?}", res.status())
    } 

    #[cfg(test)]
    pub fn _create_volume(stock: &Stock, volume: u64, price: Option<f64>) {
        use FSSM::market::market::{buy, sell, find_trades};

        buy(*stock, volume, price, None);
        sell(*stock, volume, price, None);
        find_trades(*stock);
    }
}

