use apistos::info::Info;
use apistos::spec::Spec;

pub fn api_spec() -> Spec {
    Spec {
        info: Info {
            title: "ThreatMapper Backend API".to_string(),
            version: "0.1.0".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}