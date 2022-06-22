#![deny(missing_debug_implementations)]


use std::fmt;
use sys::NSObject;
#[macro_use]
extern crate objc;


#[allow(non_camel_case_types, non_upper_case_globals, dead_code, missing_debug_implementations, non_snake_case)]
#[allow(improper_ctypes)] // Required because of blocks.
mod sys;
pub const NIL: NSObject = NSObject(std::ptr::null_mut());
impl NSObject {
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

pub mod endpoint;
pub mod parameters;
pub mod interface;
pub mod protocol;

pub enum Error {
    Posix(std::io::Error),
    Dns(DnsServiceError),
    Tls(security_framework::base::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Posix(arg0) => f.debug_tuple("Posix").field(arg0).finish(),
            Self::Dns(arg0) => f.debug_tuple("Dns").field(arg0).finish(),
            Self::Tls(arg0) => f.debug_tuple("Tls").field(arg0).finish(),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DnsServiceError {
    Unknown                   = -65537,  /* 0xFFFE FFFF */
    NoSuchName                = -65538,
    NoMemory                  = -65539,
    BadParam                  = -65540,
    BadReference              = -65541,
    BadState                  = -65542,
    BadFlags                  = -65543,
    Unsupported               = -65544,
    NotInitialized            = -65545,
    AlreadyRegistered         = -65547,
    NameConflict              = -65548,
    Invalid                   = -65549,
    Firewall                  = -65550,
    Incompatible              = -65551,  /* client library incompatible with daemon */
    BadInterfaceIndex         = -65552,
    Refused                   = -65553,
    NoSuchRecord              = -65554,
    NoAuth                    = -65555,
    NoSuchKey                 = -65556,
    NATTraversal              = -65557,
    DoubleNAT                 = -65558,
    BadTime                   = -65559,  /* Codes up to here existed in Tiger */
    BadSig                    = -65560,
    BadKey                    = -65561,
    Transient                 = -65562,
    ServiceNotRunning         = -65563,  /* Background daemon not running */
    NATPortMappingUnsupported = -65564,  /* NAT doesn't support PCP, NAT-PMP or UPnP */
    NATPortMappingDisabled    = -65565,  /* NAT supports PCP, NAT-PMP or UPnP, but it's disabled by the administrator */
    NoRouter                  = -65566,  /* No router currently configured (probably no network connectivity) */
    PollingMode               = -65567,
    Timeout                   = -65568,
    DefunctConnection         = -65569,  /* Connection to daemon returned a SO_ISDEFUNCT error result */
    PolicyDenied              = -65570,
    NotPermitted              = -65571,
}

impl TryFrom<i32> for DnsServiceError {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use DnsServiceError::*;
        match value {
            -65537 => Ok(Unknown                   ),
            -65538 => Ok(NoSuchName                ),
            -65539 => Ok(NoMemory                  ),
            -65540 => Ok(BadParam                  ),
            -65541 => Ok(BadReference              ),
            -65542 => Ok(BadState                  ),
            -65543 => Ok(BadFlags                  ),
            -65544 => Ok(Unsupported               ),
            -65545 => Ok(NotInitialized            ),
            -65547 => Ok(AlreadyRegistered         ),
            -65548 => Ok(NameConflict              ),
            -65549 => Ok(Invalid                   ),
            -65550 => Ok(Firewall                  ),
            -65551 => Ok(Incompatible              ),
            -65552 => Ok(BadInterfaceIndex         ),
            -65553 => Ok(Refused                   ),
            -65554 => Ok(NoSuchRecord              ),
            -65555 => Ok(NoAuth                    ),
            -65556 => Ok(NoSuchKey                 ),
            -65557 => Ok(NATTraversal              ),
            -65558 => Ok(DoubleNAT                 ),
            -65559 => Ok(BadTime                   ),
            -65560 => Ok(BadSig                    ),
            -65561 => Ok(BadKey                    ),
            -65562 => Ok(Transient                 ),
            -65563 => Ok(ServiceNotRunning         ),
            -65564 => Ok(NATPortMappingUnsupported ),
            -65565 => Ok(NATPortMappingDisabled    ),
            -65566 => Ok(NoRouter                  ),
            -65567 => Ok(PollingMode               ),
            -65568 => Ok(Timeout                   ),
            -65569 => Ok(DefunctConnection         ),
            -65570 => Ok(PolicyDenied              ),
            -65571 => Ok(NotPermitted              ),
            _ => Err(())
        }
    }
}