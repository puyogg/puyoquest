import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import { AWS_ACCOUNT_ID } from "../../constants.js";

export const adminRole = new aws.iam.Role("admin", {
  name: "admin",
  description: "Admin role for dev's to assume",
  assumeRolePolicy: {
    Version: "2012-10-17",
    Statement: [
      {
        Effect: "Allow",
        Principal: {
          AWS: pulumi.interpolate`arn:aws:iam::${AWS_ACCOUNT_ID}:root`,
        },
        Action: "sts:AssumeRole",
      },
    ],
  },
});

const policyAttachments = [
  "arn:aws:iam::aws:policy/AdministratorAccess",
  aws.iam.ManagedPolicy.EC2InstanceConnect,
].map((arn) => {
  const name = `AdminPolicy_${arn.split("/")[1]}`;
  return new aws.iam.RolePolicyAttachment(name, {
    role: adminRole,
    policyArn: arn,
  });
});
