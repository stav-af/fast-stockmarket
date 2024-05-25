use actix_web::{web::{self, get}, HttpResponse, Error};
use chrono::Utc;
use serde_json::json;

use crate::market::{book::*, market::*, order::{self,  OrderDetails, OrderType}};
use super::{
    request_classes::{stockmap, IpoDTO, OrderDTO, StockQuery}, 
    response_classes::{PriceDTO}
};

pub fn handle_order(req: web::Json<OrderDTO>, order_type: OrderType) -> Result<HttpResponse, Error> {
    match stockmap.get(&req.stock_name) {
        Some(stock) => {
            place_order(*stock, req.amount, order_type, req.price, None);
            Ok(HttpResponse::Ok().finish())
        },
        None => Ok(HttpResponse::NotFound().body("Stock not found")),
    }
}


pub fn handle_ipo(req: web::Json<IpoDTO>) -> Result<HttpResponse, Error> {
    let stock = stockmap.get(&req.stock_name).unwrap();
    ipo(*stock, req.amount, req.price);
    Ok(HttpResponse::Ok().finish())
}

pub fn handle_price(req: web::Query<StockQuery>) -> Result<HttpResponse, Error>{
    let stock = stockmap.get(&req.stock_name).unwrap();
    
    let res = PriceDTO {
        price: get_price(*stock),
        timestamp: Utc::now().timestamp_millis()
    };
    Ok(HttpResponse::Ok().content_type("text/plain").body(serde_json::to_string(&res).unwrap()))
}