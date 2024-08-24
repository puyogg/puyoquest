import * as awsx from "@pulumi/awsx";
import { NatGatewayStrategy } from "@pulumi/awsx/ec2/index.js";

export const ppqVpc = new awsx.ec2.Vpc("ppq-vpc", {
  assignGeneratedIpv6CidrBlock: true,
  natGateways: {
    strategy: NatGatewayStrategy.None,
  },
});
