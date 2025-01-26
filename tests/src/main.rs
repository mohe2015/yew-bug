use std::{error::Error, process::Stdio, time::Duration};

use thirtyfour::{By, DesiredCapabilities, WebDriver, prelude::ElementQueryable as _};
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    time::sleep,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut child = tokio::process::Command::new("geckodriver")
        .arg("--binary=firefox")
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .spawn()?;

    let stderr = child.stdout.take().unwrap();

    let task = tokio::spawn(async {
        let mut reader = BufReader::new(stderr).lines();

        while let Some(line) = reader.next_line().await? {
            println!("{line}");
            if line.contains("Listening on") {
                break;
            }
        }

        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        driver.goto("http://localhost:8080").await?;

        let username_input = driver.query(By::Css("#login-username")).first().await?;
        let password_input = driver.find(By::Css("#login-password")).await?;
        let login_button = driver.find(By::Css("#login-button")).await?;

        username_input.send_keys("username").await?;
        password_input.send_keys("password").await?;
        // probably https://yew.rs/docs/concepts/html/events#event-delegation
        //username_input.focus().await?;
        login_button.click().await?;

        sleep(Duration::from_secs(30)).await;

        driver.quit().await?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;

    child.kill().await?;
    child.wait().await?;
    task??;

    Ok(())
}
