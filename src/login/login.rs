use reqwest::Client;
use anyhow::anyhow;
use regex::Regex;

use super::encrypt::encrypt_aes_cbc;

/// simple regex to match input with specific attribute
/// since rust scraper is not compatible with async
async fn select_input_value_with_attr<'a>(document: &'a str, attr: &'a str, value: &'a str) -> anyhow::Result<String> {
    Regex::new(&format!(r##"<input.*{}=['"]{}['"].*value=['"](.*)['"].*[/]>"##, attr, value))?
        .captures_iter(document)
        .next()
        .map(|cap| String::from(&cap[1]))
        .ok_or_else(|| anyhow!("Match failed: attr: {}, value: {}!", attr, value))
}

/// 学工系统统一身份认证登录
///
/// ```
/// use xmu_login::create_client;
/// use xmu_login::login;
///
/// async fn test() {
///     let client = create_client("https://xmuxg.xmu.edu.cn/xmu/login?app=214").await.unwrap();
///     assert!(login(
///         &client,
///         "https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu",
///         "123123123",
///         "123123123"
///     ).await.is_err());  // username or password wrong
/// }
/// ```
pub async fn login(client: &Client, oauth_url: &str, username: &str, password: &str) -> anyhow::Result<()> {
    let login_page_resp = client
        .get(oauth_url)
        .send()
        .await?
        .text()
        .await?;

    let lt = select_input_value_with_attr(&login_page_resp, "name", "lt").await?;
    let execution = select_input_value_with_attr(&login_page_resp, "name", "execution").await?;
    let salt = select_input_value_with_attr(&login_page_resp, "id", "pwdDefaultEncryptSalt").await?;
    let password = encrypt_aes_cbc(&password, &salt);

    let post_form = [
        ("username", username),
        ("password", &password),
        ("lt", &lt),
        ("dllt", "userNamePasswordLogin"),
        ("execution", &execution),
        ("_eventId", "submit"),
        ("rmShown", "1")
    ];

    let login_resp = client
        .post(oauth_url)
        .form(&post_form)
        .send()
        .await?;

    let response_url = login_resp.url();

    if response_url.host_str() == Some("ids.xmu.edu.cn") {
        return Err(anyhow!("Login failed: username or password wrong!"));
    }

    Ok(())
}