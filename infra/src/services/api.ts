import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";

import { Ec2InstanceSsh } from "../ec2/index.js";
import { ppqVpc } from "../vpc.js";
import { AWS_ACCOUNT_ID } from "../constants.js";

const imageCacheBucket = new aws.s3.BucketV2("api-pn-image-cache", {
  bucket: "api-pn-image-cache",
});

const oac = new aws.cloudfront.OriginAccessControl("api-pn-image-cache-oac", {
  description:
    "Origin Access Control for PN Image Cache Bucket and CF Distribution",
  originAccessControlOriginType: "s3",
  signingBehavior: "always",
  signingProtocol: "sigv4",
});

const distribution = new aws.cloudfront.Distribution("api-pn-image-cache-cfd", {
  enabled: true,
  origins: [
    {
      originId: imageCacheBucket.id,
      domainName: imageCacheBucket.bucketRegionalDomainName,
      originAccessControlId: oac.id,
    },
  ],
  defaultCacheBehavior: {
    targetOriginId: imageCacheBucket.id,
    viewerProtocolPolicy: "allow-all",
    allowedMethods: ["GET", "HEAD"],
    cachedMethods: ["GET", "HEAD"],
    forwardedValues: {
      cookies: { forward: "none" },
      queryString: false,
    },
  },
  restrictions: {
    geoRestriction: {
      restrictionType: "none",
    },
  },
  viewerCertificate: {
    cloudfrontDefaultCertificate: true,
  },
});

const distributionUrl = new aws.ssm.Parameter("api-cfd-base-url", {
  name: "IMAGE_CACHE_BASE_URL",
  type: aws.ssm.ParameterType.String,
  value: pulumi.interpolate`https://${distribution.domainName}`,
});

const bucketPolicyDocument = aws.iam.getPolicyDocumentOutput({
  statements: [
    {
      effect: "Allow",
      principals: [
        {
          type: "Service",
          identifiers: ["cloudfront.amazonaws.com"],
        },
      ],
      actions: ["s3:GetObject"],
      resources: [pulumi.interpolate`${imageCacheBucket.arn}/*`],
      conditions: [
        {
          test: "StringEquals",
          variable: "AWS:SourceArn",
          values: [distribution.arn],
        },
      ],
    },
  ],
});

const imageCacheBucketPolicy = new aws.s3.BucketPolicy(
  "api-pn-image-cache-cfd-policy",
  {
    bucket: imageCacheBucket.id,
    policy: bucketPolicyDocument.apply((p) => p.json),
  }
);

export const ec2Role = new aws.iam.Role("ppq-api-ec2-role", {
  name: "ppq-api-ec2-role",
  description: "Role for EC2 instance to access S3 Bucket and DB",
  assumeRolePolicy: {
    Version: "2012-10-17",
    Statement: [
      {
        Effect: "Allow",
        Principal: {
          AWS: pulumi.interpolate`arn:aws:iam::${AWS_ACCOUNT_ID}:root`,
          Service: "ec2.amazonaws.com",
        },
        Action: "sts:AssumeRole",
      },
    ],
  },
});

const ec2RolePolicyDocument = aws.iam.getPolicyDocumentOutput({
  statements: [
    {
      effect: "Allow",
      actions: ["s3:GetObject", "s3:PutObject"],
      resources: [pulumi.interpolate`${imageCacheBucket.arn}/*`],
    },
    {
      effect: "Allow",
      actions: ["ssm:GetParameter"],
      resources: [distributionUrl.arn],
    },
  ],
});

const ec2RolePolicy = new aws.iam.RolePolicy("ppq-api-ec2-role-policy", {
  role: ec2Role.id,
  policy: ec2RolePolicyDocument.apply((p) => p.json),
});

const ec2InstanceProfile = new aws.iam.InstanceProfile("api-instance-profile", {
  role: ec2Role.name,
});

const userData = `#!/bin/bash
sudo yum update -y
sudo yum install git -y
sudo amazon-linux-extras install docker -y
sudo service docker start
sudo usermod -a -G docker ec2-user
mkdir -p /usr/local/lib/docker/cli-plugins
curl -SL https://github.com/docker/compose/releases/download/v2.29.1/docker-compose-linux-aarch64 -o /usr/local/lib/docker/cli-plugins/docker-compose
chmod +x /usr/local/lib/docker/cli-plugins/docker-compose
git clone https://github.com/puyogg/puyoquest /home/ec2-user/puyoquest
`;

const apiInstance = new Ec2InstanceSsh("api-instance", {
  vpcId: ppqVpc.vpc.id,
  subnetId: ppqVpc.publicSubnet.id,
  instanceType: "t4g.nano",
  userData,
  nameTag: "ppq-api",
  iamInstanceProfile: ec2InstanceProfile.name,
});
