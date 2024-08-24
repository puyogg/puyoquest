import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import { pulumiRequiredManagedPolicies } from "../pulumi.js";
import { AWS_ACCOUNT_ID } from "../../constants.js";

export const pulumiLocalRole = new aws.iam.Role("pulumiLocalRole", {
  name: "PulumiRoleLocal",
  description: "Role for executing pulumi locally",
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

const policyAttachments = pulumiRequiredManagedPolicies.map((arn) => {
  const name = `PulumiLocalPolicy_${arn.split("/")[1]}`;
  return new aws.iam.RolePolicyAttachment(name, {
    role: pulumiLocalRole,
    policyArn: arn,
  });
});
