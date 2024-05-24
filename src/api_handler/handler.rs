use actix_web::{web::{self, get}, HttpResponse, Error};
use crate::market::{order::{BuyOrder, OrderDetails}, book::*, market::*};
use super::classes::{OrderDTO, IpoDTO, stockmap, StockQuery};

pub fn handle_buy_order(req: web::Json<OrderDTO>) -> Result<HttpResponse, Error> {
    match stockmap.get(&req.stock_name) {
        Some(stock) => {
            match req.price {
                Some(price) => buy_limit(*stock, req.amount, price),
                None => buy_market(*stock, req.amount),
            }
            Ok(HttpResponse::Ok().finish())
        },
        None => Ok(HttpResponse::NotFound().body("Stock not found")),
    }
}


pub fn handle_sell_order(req: web::Json<OrderDTO>) -> Result<HttpResponse, Error> {
    match stockmap.get(&req.stock_name) {
        Some(stock) => {
            match req.price {
                Some(price) => sell_limit(*stock, req.amount, price),
                None => sell_market(*stock, req.amount),
            }
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
    let price = get_price(*stock);
    Ok(HttpResponse::Ok().content_type("text/plain").body(price.to_string()))
}