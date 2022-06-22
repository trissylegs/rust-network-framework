
use crate::{sys, parameters::SecProtocolOptions};

#[derive(Debug)]
pub struct TlsOptions {
    options: sys::nw_protocol_options_t
}

impl TlsOptions {
    pub fn sec_protocol_options(&mut self) -> SecProtocolOptions {
        unsafe {
            SecProtocolOptions { 
                options: sys::nw_tls_copy_sec_protocol_options(self.options)
            }
        }
    }
}