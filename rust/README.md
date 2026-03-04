# jse

JSE (JSON Structural Expression) 的 Rust 实现，可发布至 [crates.io](https://crates.io)。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
jse = "0.1"
```

## 使用

```rust
use jse::{Engine, ExpressionEnv};
use serde_json::json;

let engine = Engine::new(ExpressionEnv);

// 字面量
assert_eq!(engine.execute(&json!(42)).unwrap(), json!(42));

// 逻辑运算
let expr = json!(["$and", true, true, false]);
assert_eq!(engine.execute(&expr).unwrap(), json!(false));

// 查询模式
let query = json!({
    "$expr": ["$pattern", "$*", "author of", "$*"]
});
let sql = engine.execute(&query).unwrap().as_str().unwrap();
```

## 开发与测试

```bash
cd rust
cargo test
```

## 发布到 crates.io

1. 登录 [crates.io](https://crates.io) 并获取 API token。
2. `cargo login <token>`
3. 在 `rust/` 目录下执行：

```bash
cargo publish
```

## 仓库

<https://github.com/MarchLiu/jse>

## 许可证

MIT
