use std::time::Duration;

use dcl_rpc::{client::RpcClient, transports::web_socket::WebSocketTransport};
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use warp_dcl_rpc::{init_app, BookServiceClient, GetBookRequest, RPCServiceClient};

#[tokio::test]
async fn it_pass_the_auth_middleware() {
    tokio::spawn(async move {
        init_app().await;
    });

    sleep(Duration::from_millis(100)).await;

    let mut req = "ws://127.0.0.1:3030/ws".into_client_request().unwrap();
    req.headers_mut()
        .insert("Authorization", "123".parse().unwrap());

    let (ws_client, _) = tokio_tungstenite::connect_async(req).await.unwrap();

    let ws_transport = WebSocketTransport::new(ws_client);

    let mut rpc_client = RpcClient::new(ws_transport).await.unwrap();

    let client_port = rpc_client.create_port("TEST_port").await.unwrap();

    let book_service_client = client_port
        .load_module::<BookServiceClient<WebSocketTransport>>("BookService")
        .await
        .unwrap();

    let response = book_service_client
        .get_book(GetBookRequest { isbn: 1000 })
        .await;

    println!("Response {:?}", response);

    assert_eq!(response.title, "Rust For Rustaceans");
}

#[tokio::test]
async fn it_not_pass_the_auth_middleware() {
    tokio::spawn(async move {
        init_app().await;
    });

    sleep(Duration::from_millis(100)).await;

    let mut req = "ws://127.0.0.1:3030/ws".into_client_request().unwrap();
    req.headers_mut()
        .insert("Authorization", "1234".parse().unwrap());

    let client = tokio_tungstenite::connect_async(req).await;

    assert!(client.is_err());
    println!("{:?}", client.unwrap_err());
}
