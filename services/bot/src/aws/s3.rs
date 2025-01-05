use aws_sdk_s3::Client;
use bytes::Bytes;
use futures::StreamExt;

pub async fn get_object<S>(client: &Client, bucket: &str, key: S) -> anyhow::Result<Bytes>
where
    S: Into<String>,
{
    let key: String = key.into();

    let output = client.get_object().bucket(bucket).key(key).send().await?;

    let object = output.body.collect().await.map(|d| d.into_bytes())?;

    Ok(object)
}

pub async fn fetch_images(
    client: &Client,
    bucket: &str,
    keys: &Vec<&str>,
) -> anyhow::Result<Vec<Bytes>> {
    let get_object_futures = keys
        .iter()
        .map(|k| get_object(client, bucket, k.to_string()));
    let stream = futures::stream::iter(get_object_futures).buffered(30);
    let images = stream
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|f| f.ok())
        .collect::<Vec<Bytes>>();

    Ok(images)
}
