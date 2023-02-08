use reqwest::header;

pub async fn send_ding_notice<T: serde::Serialize>(
    url: &str,
    content: &T,
) -> Result<(), reqwest::Error> {
    let mut h = header::HeaderMap::new();
    h.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );
    let cli = reqwest::Client::builder().default_headers(h).build()?;
    let ret = cli.post(url).json(content).send().await?.text().await?;
    tracing::debug!("send ding notice result:{:?}", ret);
    Ok(())
}