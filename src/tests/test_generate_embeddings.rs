use crate::OpenAi;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_generate_embeddings_success() {
    
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let endpoint = format!("{}/v1/embeddings", url);

    let mock_response = r#"{
        "data": [
            {
                "embedding": [0.0, 0.1,0.3]
            }
        ]
    }"#;

    let mock = server.mock("POST", "/v1/embeddings")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create_async().await;

    
    let client = OpenAi::new("you_look_lonely_I_can_fix_that".to_string())
        .set_endpoint(endpoint);

    let result = client.generate_embeddings("I am lonely", "some-wonderful-embedding-model").await;

    assert!(result.is_ok());

    let embeddings_vec = result.unwrap();

    assert_eq!(embeddings_vec, vec![0.0, 0.1, 0.3]);

    mock.assert_async().await;

}