use actix_cors::Cors;
use market::order::Stock;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use std::thread;

mod market;
mod api_handler;
mod trend_generator;

use api_handler::handler;
use trend_generator::digest_cycle;

#[post("/buy")]
async fn buy(details: web::Json<api_handler::request_classes::OrderDTO>) -> Result<HttpResponse, Error> {
    handler::handle_buy_order(details)
}

#[post("/sell")]
async fn sell(details: web::Json<api_handler::request_classes::OrderDTO>) -> Result<HttpResponse, Error> {
    handler::handle_sell_order(details)
}

#[post("/ipo")]
async fn ipo(details: web::Json<api_handler::request_classes::IpoDTO>) -> Result<HttpResponse, Error> {
    handler::handle_ipo(details)
}

#[get("/price")]
async fn price(query: web::Query<api_handler::request_classes::StockQuery>) -> Result<HttpResponse, Error> {
    handler::handle_price(query)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use Stock::*;
    
    let stock_list = vec![MSFT];
    let mut mm_handles: Vec<thread::JoinHandle<_>> = Vec::new();
    for stock in stock_list {
        market::market::ipo(stock, 1, 10.0);

        let handle = thread::spawn(move || { 
            // println!("Started digesst for {:?}", stock);
            digest_cycle::make_market(stock);
        });
        mm_handles.push(handle);
    }

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
            )
            .service(buy)
            .service(sell)
            .service(ipo)
            .service(price)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}