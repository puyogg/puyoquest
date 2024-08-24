import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import { adminGroup } from "../groups/admin.js";

const user = new aws.iam.User("user-s2", {
  name: "s2",
  tags: {
    purpose: "Admin",
  },
});

const groupMembership = new aws.iam.UserGroupMembership(
  "pulumi-s2-membership",
  {
    user: user.name,
    groups: [adminGroup.name],
  }
);
