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
## Error handling

程序使用 `CliError` 区分错误：

- `Io`：文件读取或写入失败。
- `Json`：JSON 序列化或反序列化失败。
- `MissingId`：没有提供任务 ID。
- `InvalidId`：任务 ID 无法解析为 `usize`，例如字母、负数或超出范围的数字。
- `EmptyTitle`：没有提供标题，或标题全部为空白字符。
- `TaskNotFound`：指定 ID 的任务不存在。

发生错误时，程序向标准错误输出错误信息，并以退出码 `1` 结束。

当前工作目录下的 `tasks.json` 不存在时，按空任务列表处理。

## Tests

运行测试：

```bash
cargo test
```
