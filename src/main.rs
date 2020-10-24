mod client;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let uri = String::from("https://bradfitzwater.com");
    let res = client::download(uri).await?;
    dbg!(res);
    Ok(())
}
