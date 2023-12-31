pub mod huggingface;
pub mod openai;
pub use huggingface::embed;

use openai::gpt::Gpt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LanguageModel {
    Gpt(Gpt),
}

impl From<Gpt> for LanguageModel {
    fn from(value: Gpt) -> Self {
        LanguageModel::Gpt(value)
    }
}

impl LanguageModel {
    // Probably should create an into impl trait for this once more models are supported
    /// return a reference to the inner Gpt model struct
    pub fn inner_gpt(&self) -> Option<&Gpt> {
        match self {
            Self::Gpt(g) => Some(g),
            _ => None,
        }
    }
    /// Returns mutable reference to innner GPT
    pub fn inner_mut_gpt(&mut self) -> Option<&mut Gpt> {
        match self {
            Self::Gpt(g) => Some(g),
            _ => None,
        }
    }
    /// Creates LanguageModel with default gpt settings
    pub fn default_gpt() -> Self {
        let gpt = Gpt::default();
        Self::Gpt(gpt)
    }
}
