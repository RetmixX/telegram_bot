use teloxide::prelude::ChatId;
use dotenv::dotenv;
use std::env;

pub struct ConfigApp {
    pub token: String,
    pub user_id: ChatId,
}

impl ConfigApp {
    pub fn load_config() -> Self {
        Self {
            token: load_token(),
            user_id: ChatId(load_chat_id()),
        }
    }
}

fn load_token() -> String {
    dotenv().ok();
    env::var("TOKEN").expect("TOKEN not set")
}

fn load_chat_id() -> i64 {
    dotenv().ok();
    env::var("CHAT_ID").expect("CHAT_ID not set").parse::<i64>()
        .expect("CHAT_ID is not i64")
}

/*fn load_keys() -> (Keycode, Keycode) {
    dotenv().ok();
    let key_one = env::var("KEY_ONE").expect("KEY_ONE not set").as_str();
    let key_two = env::var("KEY_TWO").expect("KEY_TWO not set").as_str();

    (
        Keycode::from_str(key_one).expect("Undef key one"),
        Keycode::from_str(key_two).expect("Undef key two")
    )
}*/