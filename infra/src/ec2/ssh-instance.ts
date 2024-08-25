import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";

export class Ec2InstanceSsh extends pulumi.ComponentResource {
  private securityGroup: aws.ec2.SecurityGroup;
  public instance: aws.ec2.Instance;

  constructor(
    name: string,
    args: {
      vpcId: pulumi.Output<string>;
      subnetId: pulumi.Output<string>;
      keyPair: aws.ec2.KeyPair;
      instanceType: string;
      ipv6Only?: boolean;
      userData?: string;
    },
    opts?: pulumi.ComponentResourceOptions
  ) {
    super("infra:components:Ec2InstanceSsh", name, args, opts);

    const { vpcId, subnetId, keyPair, instanceType, ipv6Only, userData } = args;

    const ami = aws.ec2.getAmi({
      mostRecent: true,
      owners: ["amazon"],
      filters: [{ name: "name", values: ["al2023-ami-*-arm64"] }],
    });

    this.securityGroup = new aws.ec2.SecurityGroup(`${name}-sg`, {
      vpcId,
      description: "Allow inbound SSH",
      ingress: [
        {
          protocol: "tcp",
          fromPort: 22,
          toPort: 22,
          cidrBlocks: ["0.0.0.0/0"],
          ipv6CidrBlocks: ["::/0"],
        },
      ],
      egress: [
        {
          fromPort: 0,
          toPort: 0,
          protocol: "-1",
          cidrBlocks: ["0.0.0.0/0"],
          ipv6CidrBlocks: ["::/0"],
        },
      ],
    });

    this.instance = new aws.ec2.Instance(`${name}-instance`, {
      ami: ami.then((a) => a.id),
      instanceType,
      keyName: keyPair.keyName,
      subnetId,
      ipv6AddressCount: 1,
      vpcSecurityGroupIds: [this.securityGroup.id],
      userData: userData ?? "",
      userDataReplaceOnChange: true,
      associatePublicIpAddress: !ipv6Only,
    });

    this.registerOutputs({
      instance: this.instance,
      securityGroup: this.securityGroup,
    });
  }
}
