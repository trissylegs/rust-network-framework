

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Interface {
    pub name: String,
    pub index: isize,
    pub type_: InterfaceType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InterfaceType {
    Wifi,
    Cellular,
    WiredEthernet,
    Loopback,
    Other
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RadioType {
    Wifi(WifiType),
    Cell(CellularType),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum WifiType {
    B, A, G, N, AC, AX
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CellularType {
    LTE,
    WCDMA,
    GSM,
    CDMA,
    EVDO,
    Standalone5G(NewRadio5GVariant),
    DualConnectivy5G(NewRadio5GVariant),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NewRadio5GVariant {
    Sub6GHz,
    MmWave,
}
