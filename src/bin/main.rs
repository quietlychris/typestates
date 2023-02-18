use typestates::*;

fn main() {
    let net_cfg = NetworkConfig::<Tcp>::default();
    println!("{:?}", net_cfg);
    let net_cfg = NetworkConfig::<Udp>::default();

    let node_cfg = NodeConfig::<_, _>::from_net_cfg(net_cfg);
    let node = Node::<_, _, usize>::from_config(node_cfg);

    let node = Node::<Udp, Idle, usize>::default();
    let node = node.activate();
}
