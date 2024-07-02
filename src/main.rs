use std::thread;

use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Result};

mod kernel;
mod handlers;
mod globals;
mod classes;

use handlers::api_handler::*;
use classes::shared::order::*;
use classes::api::*;
use kernel::market;

#[post("/buy")]
async fn buy(details: web::Json<request_classes::OrderDTO>) -> Result<HttpResponse, Error> {
    handle_order(details, OrderType::Buy)
}

#[post("/sell")]
async fn sell(details: web::Json<request_classes::OrderDTO>) -> Result<HttpResponse, Error> {
    handle_order(details, OrderType::Sell)
}

#[post("/ipo")]
async fn ipo(details: web::Json<request_classes::IpoDTO>) -> Result<HttpResponse, Error> {
    handle_ipo(details)
}

#[get("/price")]
async fn price(query: web::Query<request_classes::StockQuery>) -> Result<HttpResponse, Error> {
    handle_price(query)
}

#[get("/stock_history")]
async fn stock_history(details: web::Json<request_classes::PriceHistoryDTO>) -> Result<HttpResponse, Error> {
   handle_stock_history(details)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use Stock::*;
    
    let stock_list = vec![MSFT];
    for stock in stock_list {
        market::ipo(stock, 1, 10.0, None);

        thread::spawn(move || { 
            // println!("Started digesst for {:?}", stock);
            kernel::agents::digest_cycle::make_market(stock);
        });
    }

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:*")
            )
            .service(buy)
            .service(sell)
            .service(ipo)
            .service(price)
            .service(stock_history)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}