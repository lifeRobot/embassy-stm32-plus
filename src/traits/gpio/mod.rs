pub mod input;
pub mod output;

/// impl gpio inout trait
#[macro_export]
macro_rules! impl_gpio_trait {
    ($p:ty) => {
        $crate::impl_gpio_input_trait!($p);
        $crate::impl_gpio_output_trait!($p);
    };
}
