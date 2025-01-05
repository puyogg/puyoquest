pub mod s3;

#[derive(Debug)]
pub struct AwsClient {
    pub s3: aws_sdk_s3::Client,
}

impl AwsClient {
    pub fn new(sdk_config: aws_config::SdkConfig) -> AwsClient {
        AwsClient {
            s3: aws_sdk_s3::Client::new(&sdk_config),
        }
    }
}
