// #[cfg(test)]
// mod tests {
//     use actix_web::{test, App};
//     use super::*;

//     #[actix_rt::test]
//     async fn test_index_get() {
//         let app = test::init_service(App::new().route("/", web::get().to(index))).await;
//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = test::call_service(&app, req).await;

//         assert!(resp.status().is_success());
//         let response_body = test::read_body(resp).await;
//         assert_eq!(response_body, "Hello, world!");
//     }
// }
