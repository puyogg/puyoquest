# puyoquest-infra

## IAM Overview

There are two Pulumi roles in the PPQ AWS account: [PulumiRoleCICD](./src/iam/pulumi.ts#L47) and [PulumiLocal](./src/iam/roles/pulumi-local.ts):

- **PulumiRoleCICD**: allows GitHub Actions to connect to the PPQ AWS account and make changes
- **PulumiLocal**: has the same permissions as PulumiRoleCICD and should be used to simulate it locally

AWS Users in the [admin group](./src/iam/groups/admin.ts) have permission to [assume](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html) the PulumiLocal role. To join the admin group, ask S2L to create a [user](./src/iam/users/) for you.

### AWS Shell Configuration

The Pulumi CLI expects your shell to have the AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY **corresponding to the PulumiLocal role** set as environment variables. These tokens are granted on a temporary basis when you assume them with the AWS CLI. To make getting these tokens more convenient, I recommend using [awsume](https://awsu.me/) with this profile configuration:

- `~/.aws/credentials`
  ```bash
  [cred-ppq-prod-you]
  aws_access_key_id = # your USER's access key id
  aws_secret_access_key = # your USER's secret access key
  mfa_serial = # arn of your user's mfa serial
  ```
- `~/.aws/config`:
  ```bash
  [profile ppq-prod-pulumi]
  region = us-east-1
  source_profile = cred-ppq-prod-you # source profile from ~/.aws/credentials
  role_arn = arn:aws:iam::<AWS_ACCOUNT_ID>:role/PulumiRoleLocal
  ```

To assume the PulumiLocal role and automatically set your AWS environment variables:

```bash
awsume ppq-prod-pulumi
```

## Executing Pulumi CLI

Setup:

```sh
awsume ppq-prod-pulumi

# This package uses ES Modules, so we need to build the JavaScript first
pnpm build

# Switch to default stack for pulumi commands
pulumi stack select puyoquest/prod
```

Then use:

- `pulumi preview`: Preview changes. Even if this succeeds, some AWS resources may have quirks that won't error until resource creation time.
- `pulumi up`: Deploy changes manually.

> WARNING: preview/up will only check the infrastructure defined by the `dist` folder, NOT `src`.
>
> For iterative development, consider compiling the TypeScript code with watch mode in a separate shell:
>
> ```sh
> pnpm build --watch
> ```
>
> Now whenever you run pulumi preview/up, it should always use your latest changes (unless it fails to type check).

## EC2 Instance SSH Access

AWS users with the admin role can SSH into EC2 instances using the [EC2 Instance Connect Endpoint](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/connect-using-eice.html). This can connect to both public and private EC2 instances.

```sh
awsume ppq-prod-admin

# Find the instance id
INSTANCE_ID=$(aws ec2 describe-instances --filters "Name=tag:name,Values=ppq-api" | jq -r '.Reservations[0].Instances[0].InstanceId')

# Use ec2-instance-connect
aws ec2-instance-connect ssh --instance-id $INSTANCE_ID --connection-type eice
```

## References

- [EC2 Instance Connect](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-connect-tutorial.html#eic-tut1-task1)
- [EC2 Instance Connect - Pulumi config](https://www.trevorrobertsjr.com/blog/ec2-instance-connect-automate/)
