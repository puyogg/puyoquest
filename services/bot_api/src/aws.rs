use aws_config::SdkConfig;

pub mod ssm;

pub struct AwsClients {
    pub ssm_client: aws_sdk_ssm::Client,
}

impl AwsClients {
    pub fn new(sdk_config: SdkConfig) -> AwsClients {
        let ssm_client = aws_sdk_ssm::Client::new(&sdk_config);

        AwsClients { ssm_client }
    }
}
