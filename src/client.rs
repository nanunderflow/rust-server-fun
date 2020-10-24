pub async fn download(uri: String) -> surf::Result<String> {
    let mut res = surf::get(uri).await?;
    res.body_string().await
}