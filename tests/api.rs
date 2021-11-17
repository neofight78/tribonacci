use reqwest::StatusCode;
use std::net::TcpListener;
use tribonacci::startup::run;

fn launch() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind to random port");
    let port = listener
        .local_addr()
        .expect("Unable to determine port")
        .port();

    let server = run(listener).expect("");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn given_an_invalid_value_a_bad_request_should_be_returned() {
    let server_url = launch();

    let response = reqwest::get(format!("{}/tribonacci/invalid", server_url))
        .await
        .expect("Unable to make request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn given_zero_a_bad_request_should_be_returned() {
    let server_url = launch();

    let response = reqwest::get(format!("{}/tribonacci/0", server_url))
        .await
        .expect("Unable to make request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn given_forty_one_a_bad_request_should_be_returned() {
    let server_url = launch();

    let response = reqwest::get(format!("{}/tribonacci/41", server_url))
        .await
        .expect("Unable to make request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn given_a_natural_number_a_result_should_be_returned() {
    let server_url = launch();

    let response = reqwest::get(format!("{}/tribonacci/10", server_url))
        .await
        .expect("Unable to make request");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.text().await.expect("Unable to read response text"),
        "44"
    );
}
