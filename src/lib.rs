#![no_std]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use arr_macro_impl::arr;
