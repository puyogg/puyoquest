import * as aws from "@pulumi/aws";

export const AWS_ACCOUNT_ID = await aws
  .getCallerIdentity()
  .then((a) => a.accountId);
