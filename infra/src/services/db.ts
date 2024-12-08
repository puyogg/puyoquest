import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

import { ppqVpc } from "../vpc.js";

const config = new pulumi.Config();

const securityGroup = new aws.ec2.SecurityGroup("db-sg", {
  vpcId: ppqVpc.vpc.id,
});

const ingressRules = [
  new aws.vpc.SecurityGroupIngressRule("db-vpc-ingress", {
    securityGroupId: securityGroup.id,
    ipProtocol: "tcp",
    fromPort: 5432,
    toPort: 5432,
    cidrIpv4: "10.0.0.0/16",
    description:
      "Allow inbound connections from other services on the same VPC",
  }),
];

const egressRules = [
  new aws.vpc.SecurityGroupEgressRule("db-vpc-egress", {
    securityGroupId: securityGroup.id,
    ipProtocol: "-1",
    fromPort: 0,
    toPort: 0,
    cidrIpv4: "0.0.0.0/0",
    description: "Allow outbound connections to anywhere on any protocol",
  }),
];

const subnetGroup = new aws.rds.SubnetGroup("db-sng", {
  subnetIds: [ppqVpc.privateSubnetA.id, ppqVpc.privateSubnetB.id],
});

export const db = new aws.rds.Instance("db", {
  dbName: "ppq_api_db",
  instanceClass: "db.t4g.micro",
  username: config.requireSecret("POSTGRES_DB_USER"),
  password: config.requireSecret("POSTGRES_DB_PASSWORD"),
  allocatedStorage: 5,
  maxAllocatedStorage: 20,
  allowMajorVersionUpgrade: true,
  backupRetentionPeriod: 14,
  engine: "postgres",
  engineVersion: "16",
  identifier: "ppq-db",
  dbSubnetGroupName: subnetGroup.name,
  vpcSecurityGroupIds: [securityGroup.id],
  skipFinalSnapshot: true,
  publiclyAccessible: false,
});

const host = db.address;
const port = db.port;
const username = config.requireSecret("POSTGRES_DB_USER");
const password = config.requireSecret("POSTGRES_DB_PASSWORD");
export const ppqConnectionString = new aws.ssm.Parameter(
  "ppq-db-connection-string",
  {
    name: "PPQ_DB_CONNECTION_STRING",
    type: aws.ssm.ParameterType.SecureString,
    value: pulumi.interpolate`postgres://${username}:${password}@${host}:${port}/ppq_api_db`,
  }
);
