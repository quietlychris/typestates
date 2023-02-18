use std::default::Default;
use std::{fmt::Debug, marker::PhantomData};

use crate::private;

pub trait Interface: private::Sealed {
    fn default() -> Self;
}

#[derive(Debug)]
pub struct Tcp {}
#[derive(Debug)]
pub struct Udp {}

impl Default for Tcp {
    fn default() -> Self {
        Tcp {}
    }
}
impl Default for Udp {
    fn default() -> Self {
        Udp {}
    }
}
impl Interface for Tcp {
    fn default() -> Self {
        <Tcp as Interface>::default()
    }
}
impl Interface for Udp {
    fn default() -> Self {
        <Udp as Interface>::default()
    }
}

pub trait Message: Debug + Sync + Send + Clone {}
impl<T> Message for T where T: Debug + Sync + Send + Clone {}

#[derive(Clone, Copy, Debug, Default)]
pub struct NetworkConfig<Interface>
where
    Interface: Default,
{
    pub __interface: PhantomData<Interface>,
    pub max_buffer_size: usize,
}

impl NetworkConfig<Tcp> {
    pub fn default() -> NetworkConfig<Tcp> {
        Self {
            __interface: PhantomData::<Tcp>,
            max_buffer_size: 1024,
        }
    }
}

impl NetworkConfig<Udp> {
    pub fn default() -> NetworkConfig<Udp> {
        Self {
            __interface: PhantomData::<Udp>,
            max_buffer_size: 2048,
        }
    }
}
