mod helpers;

use helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}