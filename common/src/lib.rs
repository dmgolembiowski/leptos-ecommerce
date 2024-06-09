//! A collection of shared type definitions and utilities
//! for the ecommerce platform.
#![allow(async_fn_in_trait, unused_imports)]
extern crate derive_builder;

#[rustfmt::skip]
use {
    serde_json as _,
    std::{
        iter::IntoIterator,
        future::Future,
        path::PathBuf,
    },
    tokio_stream::{
        self as stream, 
        Iter,
        Stream,
        StreamExt,
        StreamMap,
    },
};
mod models;
pub use models::{bill::*, customer::*, inventory::*, types::*};

// pub trait Transaction<E>: Future<Output = Result<(), E>> + Send;
// pub type FailReason = Box<dyn std::error::Error>>;

/// Describes a source of truth for the stock backing an inventory of goods,
/// in this case: cookies.
///
/// The implementor of this trait is queried for data of the associated type,
/// such that it can be used to mutate some downstream cart's composition.
pub trait Inventory {
    /// The associated type `LineItem` describes a line item in the inventory.
    /// In the case of this project, it can be an enumeration over something that
    /// resolves to a `Cookie` object.
    type LineItem;

    /// A collection which describes a customer order.
    type Order;

    /// The link necessary for the `purchase` method to relate
    /// a `Order` to an associated row in the customer table.
    type Purchaser;

    /// Returns a stream of line items that can be recovered by their domain provider
    async fn catalog(&self) -> StreamMap<InventoryRowId, Iter<std::vec::IntoIter<Self::LineItem>>>;
    // fn catalog(&self) -> impl Future<Output = StreamNotifyClose<Self::LineItem>> + Send;

    /// Applies an order to the backing inventory
    // fn purchase(&self, order: &Self::Order) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    async fn purchase(
        &self,
        order: &Self::Order,
        cust: &Self::Purchaser,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
