use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmation_without_token_rejected_with_400() {
    // Arrange
    let app = spawn_app().await;

    // Act

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}
