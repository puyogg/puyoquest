import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";
import { publicInstanceConnect } from "../vpc.js";

export class Ec2InstanceSsh extends pulumi.ComponentResource {
  public instance: aws.ec2.Instance;

  constructor(
    name: string,
    args: {
      vpcId: pulumi.Output<string>;
      subnetId: pulumi.Output<string>;
      instanceType: string;
      nameTag: string;
      isPublic?: boolean;
      ipv6Only?: boolean;
      userData?: string;
    },
    opts?: pulumi.ComponentResourceOptions
  ) {
    super("infra:components:Ec2InstanceSsh", name, args, opts);

    const {
      vpcId,
      subnetId,
      instanceType,
      ipv6Only,
      nameTag,
      isPublic,
      userData,
    } = args;

    const ami = aws.ec2.getAmi({
      mostRecent: true,
      owners: ["amazon"],
      filters: [
        { name: "name", values: ["al2023-ami-2023*-kernel-6.1-arm64"] },
      ],
    });

    this.instance = new aws.ec2.Instance(`${name}-instance`, {
      ami: ami.then((a) => a.id),
      instanceType,
      subnetId,
      ipv6AddressCount: 1,
      vpcSecurityGroupIds: [publicInstanceConnect.instanceSecurityGroup.id],
      userData: userData ?? "",
      userDataReplaceOnChange: true,
      associatePublicIpAddress: !ipv6Only,
      tags: {
        name: nameTag,
      },
    });

    this.registerOutputs({
      instance: this.instance,
    });
  }
}
