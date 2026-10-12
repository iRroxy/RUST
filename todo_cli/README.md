# todo_cli

一个简单的 Rust 命令行待办事项程序。

## Commands

添加任务：
cargo run -- add "learn Rust"
查看任务：
cargo run -- list
完成任务：
cargo run -- done 1
编辑任务：
cargo run -- edit 1 "learn more Rust"
删除任务：
cargo run -- delete 1
