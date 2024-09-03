import * as aws from "@pulumi/aws";
import * as pulumi from "@pulumi/pulumi";

export class InstanceConnect extends pulumi.ComponentResource {
  public endpoint: aws.ec2transitgateway.InstanceConnectEndpoint;
  public instanceSecurityGroup: aws.ec2.SecurityGroup;
  public endpointSecurityGroup: aws.ec2.SecurityGroup;

  constructor(
    name: string,
    args: {
      prefix: string;
      vpcId: pulumi.Output<string>;
      subnetId: pulumi.Output<string>;
    },
    opts?: pulumi.ComponentResourceOptions
  ) {
    super("infra:components:InstanceConnect", name, args, opts);

    const { prefix, vpcId, subnetId } = args;

    this.instanceSecurityGroup = new aws.ec2.SecurityGroup(`${prefix}-sg`, {
      vpcId,
    });
    this.endpointSecurityGroup = new aws.ec2.SecurityGroup(
      `${prefix}-eice-sg`,
      { vpcId }
    );

    const instanceIngress = new aws.vpc.SecurityGroupIngressRule(
      `${prefix}-sg-ingress`,
      {
        securityGroupId: this.instanceSecurityGroup.id,
        ipProtocol: "tcp",
        description: "Allow SSH connections from EC2 Instance Connect",
        fromPort: 22,
        toPort: 22,
        referencedSecurityGroupId: this.endpointSecurityGroup.id,
      }
    );
    const instanceEgress = new aws.vpc.SecurityGroupEgressRule(
      `${prefix}-sg-egress`,
      {
        securityGroupId: this.instanceSecurityGroup.id,
        ipProtocol: "-1",
        cidrIpv4: "0.0.0.0/0",
        fromPort: 0,
        toPort: 0,
      }
    );

    const endpointIngress = new aws.vpc.SecurityGroupIngressRule(
      `${prefix}-eice-sg-ingress`,
      {
        securityGroupId: this.endpointSecurityGroup.id,
        ipProtocol: "tcp",
        cidrIpv4: "0.0.0.0/0",
        // cidrIpv6: "::/0",
        description: "Allow SSH connections from anywhere",
        fromPort: 22,
        toPort: 22,
      }
    );
    const endpointEgress = new aws.vpc.SecurityGroupEgressRule(
      `${prefix}-eice-sg-egress`,
      {
        securityGroupId: this.endpointSecurityGroup.id,
        ipProtocol: "-1",
        toPort: 0,
        fromPort: 0,
        referencedSecurityGroupId: this.instanceSecurityGroup.id,
      }
    );

    this.endpoint = new aws.ec2transitgateway.InstanceConnectEndpoint(
      `${prefix}-ec2-instance-connect-endpoint`,
      {
        subnetId,
        preserveClientIp: false,
        securityGroupIds: [this.endpointSecurityGroup.id],
      },
      // By default, AWS only allows one EC2 Instance Connect Endpoint per subnet.
      // Pulumi's default behavior is to create the new resource, switch, then delete the old one,
      // which causes 2 to exist at the same time.
      { deleteBeforeReplace: true }
    );

    this.registerOutputs({
      endpoint: this.endpoint,
      instanceSecurityGroup: this.instanceSecurityGroup,
      endpointSecurityGroup: this.endpointSecurityGroup,
    });
  }
}
