pub mod wire;

use p256::PublicKey;
use std::net::Ipv6Addr;

pub struct Fabric {
    pub public_key: PublicKey,
    pub id: u64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct VID(u16);
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct PID(u16);

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct GID(u16);
pub const UGID_ALL_NODES: GID = GID(0xFFFF);
pub const UGID_ALL_NON_SLEEPY_NODES: GID = GID(0xFFFE);
pub const UGID_ALL_PROXIES: GID = GID(0xFFFD);

pub const MULTICAST_PORT: u16 = 5540;

#[repr(transparent)]
pub struct NodeID(u64);

pub fn make_multicast(fabric: &Fabric, gid: GID) -> Ipv6Addr {
    let f = fabric.id.to_be_bytes();
    let g = gid.0.to_be_bytes();
    [
        0xFF, 0x35, 0x00, 0x40, 0xFD, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], 0x00, g[0],
        g[1],
    ]
    .into()
}
