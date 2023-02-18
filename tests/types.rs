use typestates::*;

#[test]
fn test_different_network_configs() {
    let udp_cfg = NetworkConfig::<Udp>::default();
    let tcp_cfg = NetworkConfig::<Tcp>::default();
    assert!(udp_cfg.max_buffer_size != tcp_cfg.max_buffer_size);
}
