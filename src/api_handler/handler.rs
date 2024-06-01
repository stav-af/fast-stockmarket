use actix_web::{web, HttpResponse, Error};
use chrono::Utc;

use crate::market::{market::*, order::OrderType};
use super::{
    request_classes::{IpoDTO, OrderDTO, StockQuery, HistoricPriceQuery}, 
    response_classes::{PriceDTO, HistoricPriceDTO}
};

pub fn handle_order(req: web::Json<OrderDTO>, order_type: OrderType) -> Result<HttpResponse, Error> {
    match order_type {
        OrderType::Buy => buy(req.stock, req.amount,req.price, None),
        OrderType::Sell => sell(req.stock, req.amount, req.price, None)
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn handle_ipo(req: web::Json<IpoDTO>) -> Result<HttpResponse, Error> {
    ipo(req.stock, req.amount, req.price);
    Ok(HttpResponse::Ok().finish())
}

pub fn handle_price(req: web::Query<StockQuery>) -> Result<HttpResponse, Error>{
    let res = PriceDTO {
        price: get_price(req.stock),
        timestamp: Utc::now().timestamp_millis()
    };
    Ok(HttpResponse::Ok().content_type("text/plain").json(res))
}

pub fn handle_historical_price(req: web::Query<HistoricPriceQuery>) -> Result<HttpResponse, Error> {
    let data = get_historical_data(req.granularity, req.earliest_stamp, req.stock);
    match data {
        None => Ok(HttpResponse::NotFound().body("No data matching that query!")),
        Some(data) => Ok(HttpResponse::Ok().content_type("application/json").json(data))
    }
}