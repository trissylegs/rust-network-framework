
use crate::sys;

#[derive(Debug)]
pub struct TcpOptions {
    pub(crate) options: sys::nw_protocol_options_t
}

impl TcpOptions {
    pub fn new() -> TcpOptions {
        Self { options: unsafe { sys::nw_tcp_create_options() } }
    }   

    pub fn enable_fast_open(&mut self, enable_fast_open: bool) {
        unsafe {
            sys::nw_tcp_options_set_enable_fast_open(self.options, enable_fast_open)
        }
    }
}