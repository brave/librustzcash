//! Structs and methods for handling Zcash transactions.
//!
//! Brave: this module is trimmed to only what brave-core's Orchard wallet code
//! consumes via `components::orchard` (the v5 Orchard bundle serialization).
//! Upstream `zcash_primitives` builds full multi-pool (transparent / sapling /
//! sprout / tze) transactions, builders, sighash and fee logic; none of that is
//! needed here and it pulls in dependency trees we deliberately do not vendor.

pub mod components;

use corez::io::{self, Read};

use zcash_protocol::value::ZatBalance;

/// Minimal stand-in for the upstream `Transaction` type. Only the helpers
/// consumed by `components::orchard` are retained.
pub struct Transaction;

impl Transaction {
    /// Reads a little-endian `valueBalance` field as a [`ZatBalance`].
    fn read_amount<R: Read>(mut reader: R) -> io::Result<ZatBalance> {
        let mut tmp = [0; 8];
        reader.read_exact(&mut tmp)?;
        ZatBalance::from_i64_le_bytes(tmp)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "valueBalance out of range"))
    }
}
