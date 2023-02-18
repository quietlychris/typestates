use crate::net_config::*;
use crate::Message;
use std::default::Default;
use std::marker::PhantomData;

pub struct NodeConfig<I: Interface + Default, T: Message> {
    pub __data_type: PhantomData<T>,
    pub network_cfg: NetworkConfig<I>,
}

impl<I: Interface + Default, T: Message> NodeConfig<I, T> {
    pub fn default() -> NodeConfig<I, T> {
        NodeConfig {
            __data_type: PhantomData,
            network_cfg: NetworkConfig::<I>::default(),
        }
    }

    pub fn from_net_cfg(net_cfg: NetworkConfig<I>) -> NodeConfig<I, T> {
        NodeConfig {
            __data_type: PhantomData,
            network_cfg: net_cfg,
        }
    }
}
