use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "usr1",
            "password": "pwd1"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "title_str"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
