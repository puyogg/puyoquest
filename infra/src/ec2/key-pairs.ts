import * as aws from "@pulumi/aws";

export const s2Key1 = new aws.ec2.KeyPair("s2Key1", {
  publicKey:
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIEFuvqMl2DfVgzzTB5dJD70+IlxrCB/h70BXOPPCGUQ7 s2lsoftener@gmail.com",
});
