use actix_web::{web, HttpResponse, Error};
use chrono::Utc;

use crate::classes::{
    api::{request_classes::*, response_classes::*},
    shared::order::*
};
use crate::kernel::market::*;

pub fn handle_order(req: web::Json<OrderDTO>, order_type: OrderType) -> Result<HttpResponse, Error> {
    match STOCKMAP.get(&req.stock_name) {
        Some(stock) => {
            match order_type {
                OrderType::Buy => buy(*stock, req.amount,req.price, None, None),
                OrderType::Sell => sell(*stock, req.amount, req.price, None, None)
            }
            Ok(HttpResponse::Ok().finish())
        },
        None => Ok(HttpResponse::NotFound().body("Stock not found")),
    }
}


pub fn handle_ipo(req: web::Json<IpoDTO>) -> Result<HttpResponse, Error> {
    let stock = STOCKMAP.get(&req.stock_name).unwrap();
    ipo(*stock, req.amount, req.price, None);
    Ok(HttpResponse::Ok().finish())
}

pub fn handle_price(req: web::Query<StockQuery>) -> Result<HttpResponse, Error>{
    let stock = STOCKMAP.get(&req.stock_name).unwrap();
    
    let res = PriceDTO {
        price: get_price(*stock),
        timestamp: Utc::now().timestamp_millis()
    };
    Ok(HttpResponse::Ok().content_type("text/plain").body(serde_json::to_string(&res).unwrap()))
}