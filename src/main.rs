#[async_std::main]
async fn main() -> surf::Result<()> {
    let mut res = surf::get("https://bradfitzwater.com").await?;
    dbg!(res.body_string().await?);
    Ok(())
}
