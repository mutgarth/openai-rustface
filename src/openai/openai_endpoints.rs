pub enum OpenAiEndpoints{
    Embeddings,
}

impl OpenAiEndpoints {

    pub fn endpoint(&self) -> &str {
        match self {
            OpenAiEndpoints::Embeddings => "https://api.openai.com/v1/embeddings",
        }
    }

}