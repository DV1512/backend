#![cfg(test)]

use crate::server::test::actix_test;
use actix_web::test;

const INVALID_ENDPOINT: &str = "/invalid_endpoint";

actix_test!(
    fn invalid_endpoint() {
        let app = crate::server::test::init_test_app!();

        let req = test::TestRequest::get().uri(INVALID_ENDPOINT).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        Ok(())
    }
);
