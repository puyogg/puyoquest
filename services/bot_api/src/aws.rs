use aws_config::SdkConfig;

pub mod ssm;

pub struct AwsClient {
    pub ssm_client: aws_sdk_ssm::Client,
}

impl AwsClient {
    pub fn new(sdk_config: SdkConfig) -> AwsClient {
        let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

        AwsClient { ssm_client }
    }
}
