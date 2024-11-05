use poem::error::InternalServerError;
pub struct S3Client {
    pub client: aws_sdk_s3::Client,
    pub reqwest_client: reqwest::Client,
    pub image_cache_bucket_name: String,
}

impl S3Client {
    pub async fn backup_image_from_url(self, key: &str, url: &str) -> Result<(), poem::Error> {
        let pn_response = self
            .reqwest_client
            .get(url)
            .send()
            .await
            .map_err(InternalServerError)?;

        let headers = pn_response.headers();
        let content_type = headers
            .get("Content-Type")
            .and_then(|hv| hv.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let image = pn_response
            .bytes()
            .await
            .map(|i| aws_sdk_s3::primitives::ByteStream::from(i))
            .map_err(InternalServerError)?;

        let s3_response = self
            .client
            .put_object()
            .bucket(self.image_cache_bucket_name)
            .key(key)
            .body(image)
            .content_type(content_type)
            .send()
            .await
            .map_err(InternalServerError)?;

        let checksum = s3_response.checksum_sha256.unwrap_or("".to_string());
        print!("checksum {}", checksum);

        Ok(())
    }
}
