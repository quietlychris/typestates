use std::default::Default;
use std::{fmt::Debug, marker::PhantomData};

use crate::private;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub trait Interface: private::Sealed + Default {}

#[derive(Debug)]
pub struct Tcp {}
#[derive(Debug)]
pub struct Udp {}

impl Default for Udp {
    fn default() -> Self {
        Udp {}
    }
}
impl Interface for Udp {}


impl Default for Tcp {
    fn default() -> Self {
        Tcp {}
    }
}
impl Interface for Tcp {}

pub trait Message: Debug + Sync + Send + Clone {}
impl<T> Message for T where T: Debug + Sync + Send + Clone {}

#[derive(Clone, Copy, Debug)]
pub struct NetworkConfig<Interface>
where
    Interface: Default,
{
    pub __interface: PhantomData<Interface>,
    pub max_buffer_size: usize,
    pub socket: SocketAddr
}

impl<I: Interface> Default for NetworkConfig<I> {
    fn default() -> NetworkConfig<I> {
        NetworkConfig {
            __interface: PhantomData::<I>,
            max_buffer_size: 1024,
            socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25_000)
        }
    }
}

impl NetworkConfig<Tcp> {
    pub fn default() -> NetworkConfig<Tcp> {
        Self {
            __interface: PhantomData::<Tcp>,
            max_buffer_size: 1024,
            socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25_000)
        }
    }
}

impl NetworkConfig<Udp> {
    pub fn default() -> NetworkConfig<Udp> {
        Self {
            __interface: PhantomData::<Udp>,
            max_buffer_size: 2048,
            socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25_000)
        }
    }
}
