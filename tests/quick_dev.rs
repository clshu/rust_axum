#![allow(unused)]
use anyhow::Result;
use httpc_test::Error;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=Jane").await?.print().await?;

    Ok(())
}
#[tokio::test]
async fn quick_dev2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    Ok(())
}
