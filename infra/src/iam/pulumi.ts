/**
 * Creates a CI/CD role for Pulumi to assume
 *
 * References:
 * - https://www.pulumi.com/blog/managing-aws-credentials-on-cicd-part-1/
 */

import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

import { provider as GitHubOIDCProvider } from "./github.js";

const user = new aws.iam.User("cicdUser", {
  name: "cicd-bot",
  tags: {
    purpose: "Account used to perform Pulumi stack updates on CI/CD.",
  },
});

export const group = new aws.iam.Group("pulumiStackUpdaters", {
  name: "PulumiStackUpdaters",
});

const groupMembership = new aws.iam.UserGroupMembership("cicdUserMembership", {
  user: user.name,
  groups: [group.name],
});

const currentAwsIdentity = await aws.getCallerIdentity();

const groupPolicy = new aws.iam.GroupPolicy("pulumiStackUpdatersPolicy", {
  group: group.name,
  policy: {
    Version: "2012-10-17",
    Statement: [
      {
        Action: ["sts:AssumeRole"],
        Effect: "Allow",
        Resource: pulumi.interpolate`arn:aws:iam::${currentAwsIdentity.accountId}:role/*`,
        Sid: "",
      },
    ],
  },
});

const role = new aws.iam.Role(
  "PulumiRoleCICD",
  {
    name: "PulumiRoleCICD",
    description: "Pulumi AWS Role for updating stacks",
    assumeRolePolicy: {
      Version: "2012-10-17",
      Statement: [
        {
          Effect: "Allow",
          Principal: {
            // AWS: `arn:aws:iam::${AWS_ACCOUNT_ID.PUYOGG_DEV}:root`,
            Federated: GitHubOIDCProvider.arn,
          },
          Action: "sts:AssumeRoleWithWebIdentity",
          Condition: {
            StringLike: {
              "token.actions.githubusercontent.com:sub": "repo:puyogg/puyogg:*",
            },
            StringEquals: {
              "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
            },
          },
        },
      ],
    },
  },
  { dependsOn: GitHubOIDCProvider }
);

export const pulumiRequiredManagedPolicies = [
  "arn:aws:iam::aws:policy/AmazonECS_FullAccess",
  "arn:aws:iam::aws:policy/IAMFullAccess",
  "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryFullAccess",
  "arn:aws:iam::aws:policy/AmazonRDSFullAccess",
  "arn:aws:iam::aws:policy/AmazonS3FullAccess",
  "arn:aws:iam::aws:policy/AWSLambda_FullAccess",
  "arn:aws:iam::aws:policy/AmazonAPIGatewayAdministrator",
  "arn:aws:iam::aws:policy/AmazonSSMFullAccess",
  "arn:aws:iam::aws:policy/AmazonEC2FullAccess",
];

const policyAttachments = pulumiRequiredManagedPolicies.map((arn) => {
  const name = `PulumiPolicy_${arn.split("/")[1]}`;
  return new aws.iam.RolePolicyAttachment(name, {
    role,
    policyArn: arn,
  });
});
