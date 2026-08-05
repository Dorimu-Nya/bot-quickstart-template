use crate::context::fake_db::FakeDb;
use dorimubot_framework::dorimubot_commands::ReplyingMessage::Text;
use dorimubot_framework::dorimubot_commands::{CommonMessage, ReplyingMessage};

pub struct MeCmd {
    pub(crate) db: FakeDb,
}

impl MeCmd {
    pub fn me(&self, msg: &dyn CommonMessage) -> ReplyingMessage {
        let profile = self.db.get_profile(msg.get_author_openid());
        Text(format!("You are {profile}."))
    }
}
