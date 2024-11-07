use helper_macros::generate_endpoint;
use crate::services::health::check_health;

generate_endpoint! {
    fn health;
    method: get;
    path: "/health";
    docs: {
        tag: "health",
        responses: {
            (status = 200, description = "Everything works just fine!"),
            (status = 424, description = "Database not responding"),
        }
    }
    {
        check_health().await
    }
}