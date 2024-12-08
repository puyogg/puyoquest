import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";

export const apiBin = new aws.s3.BucketV2("api-bin", {
  bucket: "api-bin",
});
