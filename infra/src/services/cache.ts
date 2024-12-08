import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";
import { ppqVpc } from "../vpc.js";

const valkeySecurityGroup = new aws.ec2.SecurityGroup("ppq-valkey-sg", {
  vpcId: ppqVpc.vpc.id,
});

const sgIngress = new aws.vpc.SecurityGroupIngressRule(
  "ppq-valkey-sg-ingress",
  {
    securityGroupId: valkeySecurityGroup.id,
    ipProtocol: "tcp",
    fromPort: 6379,
    toPort: 6379,
    cidrIpv4: "10.0.0.0/16",
  }
);

const sgEgress = new aws.vpc.SecurityGroupEgressRule("ppq-valkey-sg-egress", {
  securityGroupId: valkeySecurityGroup.id,
  ipProtocol: "-1",
  fromPort: 0,
  toPort: 0,
  cidrIpv4: "0.0.0.0/0",
});

export const valkeyCache = new aws.elasticache.ServerlessCache(
  "ppq-valkey",
  {
    engine: "valkey",
    name: "ppq-cache",
    cacheUsageLimits: {
      dataStorage: {
        maximum: 3,
        unit: "GB",
      },
      ecpuPerSeconds: [
        {
          maximum: 5000,
        },
      ],
    },
    dailySnapshotTime: "00:00",
    securityGroupIds: [valkeySecurityGroup.id],
    majorEngineVersion: "8",
    snapshotRetentionLimit: 1,
    subnetIds: [
      ppqVpc.privateSubnetA.id,
      ppqVpc.privateSubnetB.id,
      // ppqVpc.publicSubnet.id,
    ],
  },
  { deleteBeforeReplace: true }
);

const valkeyAddress = valkeyCache.endpoints[0].address;
const valkeyPort = valkeyCache.endpoints[0].port;

export const redisConnectionString = new aws.ssm.Parameter(
  "redis-connection-string",
  {
    name: "REDIS_CONNECTION_STRING",
    type: aws.ssm.ParameterType.SecureString,
    value: pulumi.interpolate`rediss://:@${valkeyAddress}:${valkeyPort}`,
  }
);
