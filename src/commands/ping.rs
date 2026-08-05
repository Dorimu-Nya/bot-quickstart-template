use dorimubot_framework::dorimubot_commands::{ReplyingMessage};
use dorimubot_framework::dorimubot_commands::ReplyingMessage::Text;
use dorimubot_framework::dorimubot_commands_macros::command;

#[command("/ping")]
pub fn ping() -> ReplyingMessage {
    Text(String::from("Pong!"))
}
