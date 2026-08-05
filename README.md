# bot_quickstart_template

DorimuBot Framework 的快速开始模板。

## 项目结构

```text
.
├── Cargo.toml              # Rust 项目配置
├── config.toml             # 机器人运行配置
└── src
    ├── main.rs             # 程序入口
    ├── commands            # 指令模块
    │   ├── mod.rs
    │   ├── ping.rs
    │   └── me.rs
    └── context             # 上下文/示例数据模块
        ├── mod.rs
        └── fake_db.rs
```

## 运行

填写 `config.toml` 中的机器人凭据后执行：

```shell
cargo run
```
