pub mod commands;
pub mod context;

use crate::commands::me::MeCmd;
use crate::context::fake_db::FakeDb;
use config::Config;
use dorimubot_framework::{
    dorimubot_commands::{CommandPlugin, CommonMessage},
    dorimubot_framework_core::{QQBot, QQBotConfig},
    run_dorimubot,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "debug"); }
    }

    let config = Config::builder()
        .add_source(::config::File::with_name("config.toml").required(false))
        .build();
    let config: QQBotConfig = config?.try_deserialize()?;

    let db = FakeDb {};
    let me_cmd = MeCmd { db };
    let command_plugin = CommandPlugin::new()
        .with_command("/me", move |msg: &dyn CommonMessage| me_cmd.me(msg));

    let app = QQBot::new(config);
    command_plugin.register(&app);
    run_dorimubot(app).await?;
    Ok(())
}
