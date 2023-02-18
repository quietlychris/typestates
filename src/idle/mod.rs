use std::default::Default;
use std::marker::PhantomData;

use crate::net_config::*;
use crate::*;

impl<I: Interface + Default, T: Message> Node<I, Idle, T> {
    pub fn from_config(cfg: NodeConfig<I, T>) -> Node<I, Idle, T> {
        Node::<I, Idle, T> {
            __state: PhantomData::<Idle>,
            cfg,
        }
    }

    pub fn default() -> Node<I, Idle, T> {
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

    pub fn activate(self) -> Node<I, Active, T> {
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
