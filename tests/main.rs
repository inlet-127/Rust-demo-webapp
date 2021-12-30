#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Test
    let resBody = response.text().await;
    match resBody {
        Ok(x) => (),
        Err(e) => panic!("test faild"),
    }
}
#[tokio::test]
async fn great_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/demo/hello")
        .send()
        .await
        .expect("Failed to execute request.");
    // Test
    let resBody = response.text().await;
    match resBody {
        Ok(x) => assert_eq!("Hello hello!", x),
        Err(e) => panic!("test faild"),
    }
}
// webapp_demoをバックグラウンド実行する
fn spawn_app() {
    let server = webapp_demo::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
