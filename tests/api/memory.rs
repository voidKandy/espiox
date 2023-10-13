use crate::helpers::{init_test, test_gpt};
use espionox::{
    agent::Agent,
    context::memory::{CachingMechanism, Memory, MessageRole, MessageVector, ToMessage},
    language_models::LanguageModel,
};

#[ignore]
#[tokio::test]
async fn oversized_cache_handled_correctly() {
    init_test();
    let limit = 4;
    let mech = CachingMechanism::SummarizeAtLimit {
        limit,
        save_to_lt: false,
    };
    let memory = Memory::build().caching_mechanism(mech).finished();
    let model = LanguageModel::from(test_gpt());
    let mut agent = Agent { memory, model };
    for _ in 0..=3 {
        agent
            .memory
            .push_to_message_cache("user", "Hello".to_string())
            .await;
        agent
            .memory
            .push_to_message_cache("assistant", "Hello! how can i help you?".to_string())
            .await;
    }
    assert!(limit >= agent.memory.cache().len_excluding_system_prompt());
}