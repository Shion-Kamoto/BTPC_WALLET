use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn e2e_send_and_broadcast_stub() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/address/addrX/utxos"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(vec![json!({ "txid": "a", "vout": 0, "value": 1_000 })]),
        )
        .mount(&server)
        .await;

    assert!(server.address().ip().is_ipv4());
}
