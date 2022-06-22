/*
#[derive(Debug)]
#[repr(C)]
pub struct __nw_parameters(libc::c_void);

pub type nw_parameters_t = *const __nw_parameters;

#[derive(Debug)]
#[repr(C)]
pub struct __nw_parameters_configure_protocol_block(libc::c_void);

pub type nw_parameters_configure_protocol_block_t = *const __nw_parameters_configure_protocol_block;

extern "C" {
    static _nw_parameters_configure_protocol_default_configuration:
        nw_parameters_configure_protocol_block_t;
    static _nw_parameters_configure_protocol_disable: nw_parameters_configure_protocol_block_t;

    fn nw_parameters_create_secure_tcp(
        configure_tls: nw_parameters_configure_protocol_block_t,
        configure_tcp: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;

    fn nw_parameters_create_secure_udp(
        configure_dtls: nw_parameters_configure_protocol_block_t,
        configure_udp: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;

    fn nw_parameters_create_custom_ip(
        custom_ip_protocol_number: u8,
        configure_ip: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;

    fn nw_parameters_create_quic(
        configure_quic: nw_parameters_configure_protocol_block_t,
    ) -> nw_parameters_t;

    fn nw_parameters_create_application_service() -> nw_parameters_t;

    fn nw_parameters_create() -> nw_parameters_t;

    fn nw_parameters_copy(parameters: nw_parameters_t) -> nw_parameters_t;
}
*/

use std::ptr;

use block::ConcreteBlock;
use libc::c_void;

use crate::{sys::{self, nw_parameters_t, nw_release}, protocol::tcp::TcpOptions, NIL};

#[derive(Debug)]
pub struct Parameters {
    parameters: nw_parameters_t,
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        Self {
            parameters: unsafe { sys::nw_parameters_copy(self.parameters) },
        }
    }
}

pub trait ConfigureProtocolOptions<T>: 'static {
    fn configure(&self, options: &mut T);
}

#[derive(Debug)]
pub struct DefaultOptions { _private: () }
pub const DEFAULT_OPTIONS: DefaultOptions = DefaultOptions { _private: () };
impl<T> ConfigureProtocolOptions<T> for DefaultOptions {
    fn configure(&self, _options: &mut T) {}
}

impl<F,T> ConfigureProtocolOptions<T> for F
    where F: Fn(&mut T) + 'static
{
    fn configure(&self, options: &mut T) {
        (self)(options)
    }
}

impl Parameters {
    pub fn new() -> Self {
        Parameters { parameters: unsafe { sys::nw_parameters_create() } }
    }

    pub fn secure_tcp<F1, F2>(configure_tls: F1, configure_tcp: F2 ) -> Self 
        where F1: ConfigureProtocolOptions<TlsOptions>, F2: ConfigureProtocolOptions<TcpOptions>
    {
        let configure_tls_block = ConcreteBlock::new(move |(options,)| configure_tls.configure(&mut TlsOptions { options }));
        let configure_tls_block = configure_tls_block.copy();
        let configure_tls_block: sys::nw_parameters_configure_protocol_block_t = &configure_tls_block as *const _ as *mut _;
        let configure_tcp_block = ConcreteBlock::new(move |(options,)| configure_tcp.configure(&mut TcpOptions { options }));
        let configure_tcp_block = configure_tcp_block.copy();
        let configure_tcp_block: sys::nw_parameters_configure_protocol_block_t = &configure_tcp_block as *const _ as *mut _;

        let parameters = unsafe {
            sys::nw_parameters_create_secure_tcp(configure_tls_block, configure_tcp_block)
        };
        Self { parameters }
    }

    pub fn dtls() -> Self {
        todo!()
    }

    pub fn udp() -> Self {
        todo!()
    }

    pub fn quic(_alpn: Vec<String>) -> Self {
        todo!()
    }

    pub fn quic_datagram(_alpn: Vec<String>) -> Self {
        todo!()
    }

    pub fn default_protocol_stack(&self) -> ProtocolStack {
        unsafe {
            ProtocolStack { protcol_stack: sys::nw_parameters_copy_default_protocol_stack(self.parameters) }
        }
    }
}

#[derive(Debug)]
pub struct ProtocolStack {
    protcol_stack: sys::nw_protocol_stack_t
}

impl Drop for ProtocolStack {
    fn drop(&mut self) {
        if !self.protcol_stack.is_null() {
            unsafe {
                nw_release(self.protcol_stack.0 as *mut c_void);
            }
        }
    }
}

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

#[derive(Debug)]
pub struct SecProtocolOptions {
    pub(crate) options: sys::sec_protocol_options_t,
}

impl Drop for SecProtocolOptions {
    fn drop(&mut self) {
        unsafe {
            if !self.options.is_null() {
                sys::nw_release(self.options.0 as *mut c_void);
                self.options = NIL;
            }
        }
    }
}

impl SecProtocolOptions {
    pub fn add_pre_shared_key(&self, keydata: impl AsRef<[u8]>) {
        let key = keydata.as_ref();
        unsafe {

            let psk = sys::dispatch_data_create(
                key.as_ptr() as *const c_void, key.len() as u64, 
                NIL, 
                // You're supposed to use DISPATCH_DATA_DESTRUCTOR_DEFAULT. But it's #define'd to NULL so this will work.
                ptr::null());            
            sys::sec_protocol_options_add_pre_shared_key(self.options, psk, psk);
            sys::dispatch_release(psk);
        }         
    }

    pub fn add_tls_ciphersuite(&self, suite: security_framework::cipher_suite::CipherSuite) {
        let raw = suite.to_raw();
        unsafe {
            sys::sec_protocol_options_add_tls_ciphersuite(self.options, raw);
        }
    }
}
