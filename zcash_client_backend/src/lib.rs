//! *A crate for implementing Zcash light clients.*
//!
//! `zcash_client_backend` contains Rust structs and traits for creating shielded Zcash
//! light clients.
//!
//! # Design
//!
//! ## Wallet sync
//!
//! The APIs in the [`data_api::chain`] module can be used to implement the following
//! synchronization flow:
//!
//! ```text
//!                          ┌─────────────┐  ┌─────────────┐
//!                          │Get required │  │   Update    │
//!                          │subtree root │─▶│subtree roots│
//!                          │    range    │  └─────────────┘
//!                          └─────────────┘         │
//!                                                  ▼
//!                                             ┌─────────┐
//!                                             │ Update  │
//!           ┌────────────────────────────────▶│chain tip│◀──────┐
//!           │                                 └─────────┘       │
//!           │                                      │            │
//!           │                                      ▼            │
//!    ┌─────────────┐        ┌────────────┐  ┌─────────────┐     │
//!    │  Truncate   │        │Split range │  │Get suggested│     │
//!    │  wallet to  │        │into batches│◀─│ scan ranges │     │
//!    │rewind height│        └────────────┘  └─────────────┘     │
//!    └─────────────┘               │                            │
//!           ▲                     ╱│╲                           │
//!           │      ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─              │
//!      ┌────────┐         ┌───────────────┐       │             │
//!      │ Choose │  │      │Download blocks│                     │
//!      │ rewind │         │   to cache    │       │             │
//!      │ height │  │      └───────────────┘           .───────────────────.
//!      └────────┘                 │               │  ( Scan ranges updated )
//!           ▲      │              ▼                   `───────────────────'
//!           │               ┌───────────┐         │             ▲
//!  .───────────────┴─.      │Scan cached│    .─────────.        │
//! ( Continuity error  )◀────│  blocks   │──▶(  Success  )───────┤
//!  `───────────────┬─'      └───────────┘    `─────────'        │
//!                                 │               │             │
//!                  │       ┌──────┴───────┐                     │
//!                          ▼              ▼       │             ▼
//!                  │┌─────────────┐┌─────────────┐  ┌──────────────────────┐
//!                   │Delete blocks││   Enhance   ││ │Update wallet balance │
//!                  ││ from cache  ││transactions │  │  and sync progress   │
//!                   └─────────────┘└─────────────┘│ └──────────────────────┘
//!                  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
//! ```
//!
//!

// Catch documentation errors caused by code changes.
#![deny(rustdoc::broken_intra_doc_links)]

// Brave: only the shardtree serialization helpers are used by brave-core's
// Orchard shard-tree storage. The rest of the crate (data access, scanning,
// gRPC proto, sapling/transparent support) is trimmed to avoid pulling in the
// full light-client dependency surface.
pub mod serialization;
