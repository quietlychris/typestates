use std::{fmt::Debug, marker::PhantomData};
use std::default::Default;

fn main() {
    let net_cfg = NetworkConfig::<Tcp>::default();
    println!("{:?}",net_cfg);
    let node_cfg = NodeConfig::<_, _>::from_net_cfg(net_cfg);
    let node = Node::<_, _, usize>::from_config(node_cfg);

    let node = Node::<Udp, Idle, usize>::default();
    let node = node.activate();
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Udp {}
    impl Sealed for crate::Tcp {}

    impl Sealed for crate::Idle {}
    impl Sealed for crate::Active {}
}

pub trait Interface: private::Sealed {
    fn default() -> Self;
}

#[derive(Debug)]
struct Tcp {}
#[derive(Debug)]
struct Udp {}

impl Default for Tcp {
    fn default() -> Self { Tcp {} }
}
impl Default for Udp {
    fn default() -> Self { Udp {} }
}
impl Interface for Tcp {
    fn default() -> Self { <Tcp as Interface>::default()}
}
impl Interface for Udp {
    fn default() -> Self {<Udp as Interface>::default()}
}

pub trait Message: Debug + Sync + Send + Clone {}
impl<T> Message for T where T: Debug + Sync + Send + Clone {}

#[derive(Clone, Copy, Debug, Default)]
struct NetworkConfig<Interface> where Interface: Default{
    __interface: PhantomData<Interface>,
    max_buffer_size: usize,
}

impl NetworkConfig<Tcp> {
    fn default() -> NetworkConfig<Tcp> {
        Self {
            __interface: PhantomData::<Tcp>,
            max_buffer_size: 1024
        }
    }
}

impl NetworkConfig<Udp> {
    fn default() -> NetworkConfig<Udp> {
        Self {
            __interface: PhantomData::<Udp>,
            max_buffer_size: 2048
        }
    }
}

struct NodeConfig<I: Interface + Default, T: Message> {
    __data_type: PhantomData<T>,
    network_cfg: NetworkConfig<I>,
}

impl<I: Interface + Default, T: Message> NodeConfig<I, T> {
    fn default() -> NodeConfig<I, T> {
        NodeConfig {
            __data_type: PhantomData,
            network_cfg: NetworkConfig::<I>::default(),
        }
    }

    fn from_net_cfg(net_cfg: NetworkConfig<I>) -> NodeConfig<I, T> {
        NodeConfig {
            __data_type: PhantomData,
            network_cfg: net_cfg,
        }
    }
}

struct Node<I: Interface + Default, State, T: Message> {
    pub __state: PhantomData<State>,
    cfg: NodeConfig<I, T>,
}

trait State: Default + Clone + private::Sealed {}
#[derive(Default, Clone)]
struct Idle;
#[derive(Default, Clone)]
struct Active;

impl State for Idle {}
impl State for Active {}

impl<I: Interface + Default, T: Message> Node<I, Idle, T> {
    fn from_config(cfg: NodeConfig<I, T>) -> Node<I, Idle, T> {
        Node::<I, Idle, T> {
            __state: PhantomData::<Idle>,
            cfg: NodeConfig::<I, T> {
                __data_type: PhantomData::<T>,
                network_cfg: NetworkConfig::<I> {
                    __interface: PhantomData::<I>,
                    max_buffer_size: 1_000,
                },
            },
        }
    }

    fn default() -> Node<I, Idle, T> {
        Node::<I, Idle, T> {
            __state: PhantomData::<Idle>,
            cfg: NodeConfig::<I, T> {
                __data_type: PhantomData::<T>,
                network_cfg: NetworkConfig::<I> {
                    __interface: PhantomData::<I>,
                    max_buffer_size: 1_000,
                },
            },
        }
    }

    fn activate(self) -> Node<I, Active, T> {
        Node::<I, Active, T> {
            __state: PhantomData::<Active>,
            cfg: NodeConfig::<I, T> {
                __data_type: PhantomData::<T>,
                network_cfg: NetworkConfig::<I> {
                    __interface: PhantomData::<I>,
                    max_buffer_size: 1_000,
                },
            },
        }
    }
}
