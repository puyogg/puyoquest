use aws_sdk_s3::Client;
use bytes::Bytes;

pub const IMAGE_CACHE_BUCKET_NAME: &'static str = "api-pn-image-cache";

pub async fn get_object<S>(client: &Client, bucket: &str, key: S) -> anyhow::Result<Bytes>
where
    S: Into<String>,
{
    let key: String = key.into();

    let output = client.get_object().bucket(bucket).key(key).send().await?;

    let object = output.body.collect().await.map(|d| d.into_bytes())?;

    Ok(object)
}
