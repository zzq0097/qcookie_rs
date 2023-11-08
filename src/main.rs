use std::env;
use std::process::Command;

use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use thirtyfour::prelude::*;
use tokio::time::{Duration, sleep};

const LOGIN_URL: &str = "https://graph.qq.com/oauth2.0/authorize?response_type=code&client_id=100497308&redirect_uri=https%3A%2F%2Fy.qq.com%2Fportal%2Fwx_redirect.html%3Flogin_type%3D1%26surl%3Dhttps%253A%252F%252Fy.qq.com%252Fportal%252Fradio.html%26use_customer_cb%3D0&state=state&display=pc";


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().unwrap();

    let u = env::var("u").unwrap();
    let p = env::var("p").unwrap();
    let driver_path = env::var("driver_path").unwrap();
    let qq_music_api = env::var("qq_music_api").unwrap();

    let mut chromedriver_process = Command::new(driver_path)
        .arg("-p")
        .arg("9515")
        .spawn().expect("无法启动driver");

    let driver = WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome()).await?;

    println!("打开网页");
    driver.goto(LOGIN_URL).await?;
    println!("切换iframe");
    driver.enter_frame(0).await?;

    println!("从二维码登录切换到密码登录");
    driver.find(By::Id("switcher_plogin")).await?.click().await?;

    sleep(Duration::from_secs(1)).await;
    let u_text = driver.find(By::Id("u")).await?;
    println!("输入用户名");
    u_text.send_keys(u).await?;

    sleep(Duration::from_secs(1)).await;
    let p_text = driver.find(By::Id("p")).await?;
    println!("输入密码");
    p_text.send_keys(p).await?;

    sleep(Duration::from_secs(1)).await;
    println!("点击登录");
    driver.find(By::Id("login_button")).await?.click().await?;

    sleep(Duration::from_secs(10)).await;
    let cookies = driver.get_all_cookies().await?;
    let cookie_str = cookies.iter()
        .filter(|cookie| !cookie.value().is_empty())
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<String>>().join(";");

    println!("cookie_str: {}", cookie_str);

    send_http(cookie_str, qq_music_api).await;

    sleep(Duration::from_secs(10)).await;

    driver.quit().await?;

    chromedriver_process.kill().expect("无法关闭driver");
    Ok(())
}



async fn send_http(cookie_str: String, qq_music_api: String) {
    let client = Client::new();
    let json_body = json!({
        "data": cookie_str
    }).to_string();

    let response = client.post(qq_music_api + "/user/setCookie")
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }
}