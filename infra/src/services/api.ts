import { Ec2InstanceSsh } from "../ec2/index.js";
import { ppqVpc } from "../vpc.js";
import { s2Key1 } from "../ec2/index.js";

const userData = `#!/bin/bash
sudo yum update -y
sudo yum install git -y
sudo amazon-linux-extras install docker -y
sudo service docker start
sudo usermod -a -G docker ec2-user
mkdir -p /usr/local/lib/docker/cli-plugins
curl -SL https://github.com/docker/compose/releases/download/v2.29.1/docker-compose-linux-aarch64 -o /usr/local/lib/docker/cli-plugins/docker-compose
chmod +x /usr/local/lib/docker/cli-plugins/docker-compose
git clone https://github.com/puyogg/puyoquest /home/ec2-user/puyoquest
`;

const apiInstance = new Ec2InstanceSsh("api-instance", {
  vpcId: ppqVpc.vpc.id,
  subnetId: ppqVpc.publicSubnet.id,
  keyPair: s2Key1,
  instanceType: "t4g.nano",
  userData,
});
