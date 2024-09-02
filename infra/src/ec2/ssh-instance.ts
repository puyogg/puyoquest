import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";

export class Ec2InstanceSsh extends pulumi.ComponentResource {
  private securityGroup: aws.ec2.SecurityGroup;
  private eiceSecurityGroup: aws.ec2.SecurityGroup;
  private ec2InstanceConnectEndpoint:
    | aws.ec2transitgateway.InstanceConnectEndpoint
    | undefined;
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

    this.securityGroup = new aws.ec2.SecurityGroup(`${name}-sg`, {
      vpcId,
      description:
        "Allow SSH connections from EC2 Instance Connect and outbound connections to anywhere.",
    });

    this.eiceSecurityGroup = new aws.ec2.SecurityGroup(`${name}-eice-sg`, {
      vpcId,
    });

    const securityGroupIngress = new aws.vpc.SecurityGroupIngressRule(
      `${name}-sg-ingress`,
      {
        securityGroupId: this.securityGroup.id,
        ipProtocol: "tcp",
        // cidrIpv4: "0.0.0.0/0",
        description: "Allow SSH connections from EC2 Instance Connect",
        fromPort: 22,
        toPort: 22,
        referencedSecurityGroupId: this.eiceSecurityGroup.id,
      }
    );

    if (isPublic) {
      // TODO
    }

    const securityGroupEgress = new aws.vpc.SecurityGroupEgressRule(
      `${name}-sg-egress`,
      {
        securityGroupId: this.securityGroup.id,
        ipProtocol: "-1",
        cidrIpv4: "0.0.0.0/0",
        // cidrIpv6: "::/0",
        fromPort: 0,
        toPort: 0,
      }
    );

    const eiceIngress = new aws.vpc.SecurityGroupIngressRule(
      `${name}-eice-sg-ingress`,
      {
        securityGroupId: this.eiceSecurityGroup.id,
        ipProtocol: "tcp",
        cidrIpv4: "0.0.0.0/0",
        // cidrIpv6: "::/0",
        description: "Allow SSH connections from anywhere",
        fromPort: 22,
        toPort: 22,
      }
    );
    const eiceEgress = new aws.vpc.SecurityGroupEgressRule(
      `${name}-eice-sg-egress-ec2`,
      {
        securityGroupId: this.eiceSecurityGroup.id,
        ipProtocol: "-1",
        toPort: 0,
        fromPort: 0,
        referencedSecurityGroupId: this.securityGroup.id,
      }
    );

    this.instance = new aws.ec2.Instance(`${name}-instance`, {
      ami: ami.then((a) => a.id),
      instanceType,
      subnetId,
      ipv6AddressCount: 1,
      vpcSecurityGroupIds: [this.securityGroup.id],
      userData: userData ?? "",
      userDataReplaceOnChange: true,
      associatePublicIpAddress: !ipv6Only,
      tags: {
        name: nameTag,
      },
    });

    this.ec2InstanceConnectEndpoint =
      new aws.ec2transitgateway.InstanceConnectEndpoint(
        "${name}-ec2-instance-connect",
        {
          subnetId,
          preserveClientIp: false,
          securityGroupIds: [this.eiceSecurityGroup.id],
        },
        // By default, AWS only allows one EC2 Instance Connect Endpoint per subnet.
        // Pulumi's default behavior is to create the new resource, switch, then delete the old one,
        // which causes 2 to exist at the same time.
        { deleteBeforeReplace: true }
      );

    this.registerOutputs({
      instance: this.instance,
      securityGroup: this.securityGroup,
      eiceSecurityGroup: this.eiceSecurityGroup,
      ec2InstanceConnectEndpoint: this.ec2InstanceConnectEndpoint,
    });
  }
}
