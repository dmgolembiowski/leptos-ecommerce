#[rustfmt::skip]
use {
    serde_json as _,
    std::iter::IntoIterator,
};

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

    /// Returns an iterator  
    fn catalog(&self) -> impl IntoIterator<Item = Self::LineItem>;

    /// Applies an order to the backing inventory
    fn purchase(
        &mut self,
        _order: impl AsRef<Self::Order>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err("Not yet implemented".into())
    }
}
