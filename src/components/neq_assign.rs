//! `neq_assign`, idea from [Yew's documentation](https://yew.rs/docs/advanced-topics/optimizations)
//!
//! ## Usage
//!
//! ```ignore
//! fn change(&mut self, props: Self::Properties) -> ShouldRender {
//!     self.props.neq_assign(props)
//! }
//! ```
//!

use yew::html::ShouldRender;

// TODO: Use yewtils's one
pub trait NeqAssign {
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}
