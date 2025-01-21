use soroban_sdk::{ Env, String, Symbol};

pub fn executed(
    env: &Env,
    source_chain: String,
    message_id: String,
    source_address: String,
    payload: String,
) {
    let topics = (
        Symbol::new(env, "executed"),
        source_chain,
        message_id,
        source_address,
    );

    env.events().publish(topics, (payload,));
}
