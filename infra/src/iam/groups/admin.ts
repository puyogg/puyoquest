import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import { AWS_ACCOUNT_ID } from "../../constants.js";
import { adminRole } from "../roles/admin.js";
import { pulumiLocalRole } from "../roles/pulumi-local.js";

export const adminGroup = new aws.iam.Group("adminGroup", {
  name: "AdminGroup",
});

const groupPolicy = new aws.iam.GroupPolicy("adminGroupPolicy", {
  group: adminGroup.name,
  policy: {
    Version: "2012-10-17",
    Statement: [
      {
        Action: ["sts:AssumeRole"],
        Effect: "Allow",
        Resource: adminRole.arn,
        Sid: "",
      },
      {
        Action: ["sts:AssumeRole"],
        Effect: "Allow",
        Resource: pulumiLocalRole.arn,
        Sid: "",
      },
    ],
  },
});
