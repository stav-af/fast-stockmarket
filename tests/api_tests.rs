use FSSM::api_handler::{request_classes::*, handler::*};
use FSSM::market::order::OrderType::*;
use FSSM::market::market::ipo;
use FSSM::market::order::Stock::*;
use FSSM::market::order::Stock;

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, thread, time::Duration};

    use super::*;

    use FSSM::{api_handler::response_classes::{PriceDTO, HistoricPriceDTO}, trend_generator::digest_cycle};
    use actix_web::{test, web, http::{self}, App, HttpResponse, body::to_bytes};
    use serde_json::json;
    #[test]
    async fn test_happypath_price_query() {
        ipo(MSFT, 10000, 10.0);
        digest_cycle::make_market(MSFT);
        let query = StockQuery {
            stock: MSFT
        };
        let req = web::Query(query);

        thread::sleep(Duration::from_nanos(100));
        let res: HttpResponse = handle_price(req).expect("error?");
        // Convert response body to bytes
        let body_bytes = to_bytes(res.into_body()).await.unwrap();
        let body: PriceDTO = serde_json::from_slice(&body_bytes).unwrap();

        assert!(body.price == 10.0, "Expected body.price to be 10.0, but found {}", body.price);
    }

    #[test]
    async fn test_happypath_historic_price_query() {
        ipo(MSFT, 10000, 10.0);
        digest_cycle::make_market(MSFT);
        let query = HistoricPriceQuery {
            stock:MSFT, 
            granularity: FSSM::globals::GRANULARITY::SECOND, 
            earliest_stamp: 0
        };
        let req = web::Query(query);

        thread::sleep(Duration::from_nanos(100000000));
        let res: HttpResponse = handle_historic_price(req).expect("error?");
        // Convert response body to bytes
        let body_bytes = to_bytes(res.into_body()).await.unwrap();
        let body: HistoricPriceDTO = serde_json::from_slice(&body_bytes).unwrap();

        for datapoint in &body.data {
            println!("{:?}", datapoint);
        }

        assert!(body.data.iter().all(|o| o.timestamp > 0), "Timestamp found greater than 0");
        assert!(body.data.iter().all(|o| o.volume > 0), "Volume found at 0");
        assert!(body.granularity as i64 == FSSM::globals::GRANULARITY::SECOND as i64, "Expected request granularity to equal response granularity");
        assert!(body.data.iter().all(|o| o.max_price >= o.min_price), "Max price > min price");

        for i in 0..body.data.len() - 1 {
            let x = body.data[i].timestamp;
            let y = body.data[i+1].timestamp;
            println!("X: {}, Y: {}, Diff: {}", x, y, x - y);
        }
    }  
}

