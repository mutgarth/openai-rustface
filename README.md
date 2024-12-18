# openai-rustface

This repository provides a simple Rust client to interact with the OpenAI API. At the moment only the embeddings endpoint is enabled.

## Features

- [x] Generate embeddings for given text.
- [ ] Chat completions endpoint.

## Requirements

- Rust: Ensure you have Rust installed. If not, download it from rust-lang.org.
- OpenAI API Key: Obtain an API key by signing up at OpenAI. You will need to fund your account before using it.

## usage

use openai-rustface::OpenAi;

```
#[tokio::main]
async fn main() {
    // Replace "your_openai_api_key" with your actual API key
    let api_key = "your_openai_api_key".to_string();
    let openai_client = OpenAi::new(api_key);

    match openai_client
        .generate_embeddings("Hello, world!", "text-embedding-ada-002")
        .await
    {
        Ok(embeddings) => println!("Embeddings: {:?}", embeddings),
        Err(e) => eprintln!("Error generating embeddings: {:?}", e),
    }
}
```
## Running tests

At the moment only one test was implemented. It uses mockito to mock the HTTP Post request. 

To run the tests:

```
cargo test
``` 
