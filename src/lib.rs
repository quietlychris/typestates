use std::default::Default;
use std::marker::PhantomData;

mod config;
mod idle;
mod net_config;

use crate::net_config::*;
pub use crate::net_config::{NetworkConfig, Tcp, Udp};

pub use crate::config::*;
pub use crate::idle::*;

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Udp {}
    impl Sealed for crate::Tcp {}

    impl Sealed for crate::Idle {}
    impl Sealed for crate::Active {}
}

pub struct Node<I: Interface + Default, State, T: Message> {
    pub __state: PhantomData<State>,
    pub cfg: NodeConfig<I, T>,
}

trait State: Default + Clone + private::Sealed {}
#[derive(Default, Clone)]
pub struct Idle;
#[derive(Default, Clone)]
pub struct Active;

impl State for Idle {}
impl State for Active {}
