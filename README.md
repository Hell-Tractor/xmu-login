# XMU Login Helper

[![LICENSE](https://img.shields.io/github/license/Hell-Tractor/xmu-login)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/xmu-login)](https://crates.io/crates/xmu-login)
[![docs.rs](https://img.shields.io/docsrs/xmu-login)](https://docs.rs/xmu-login)

一个用于完成厦门大学统一身份认证的Rust语言SDK。

本项目由[XMU Daily Health Report - Rust CLI & SDK](https://github.com/Hell-Tractor/auto-daily-health-report)衍生而来，提取了原项目中的登录部分，并对依赖进行了更新。

## Usage

```rust
use xmu_login::create_client;
use xmu_login::login;

async fn some_function() {
    let client = create_client("https://xmuxg.xmu.edu.cn/xmu/login?app=214")
        .await
        .unwrap();

    assert!(login(
        &client,
        "https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu",
        "123123123",
        "123123123"
    ).await.is_err());  // username or password wrong
}
```