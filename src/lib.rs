// #![no_std]
// #![allow(async_fn_in_trait)]
// #![allow(unknown_lints)]
// #![allow(renamed_and_removed_lints)]
// #![allow(clippy::unused_unit)] // enumset
// #![allow(unexpected_cfgs)]
// #![warn(clippy::large_futures)]
// #![cfg_attr(feature = "nightly", feature(doc_cfg))]
// #![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

// #[cfg(not(esp_idf_comp_driver_enabled))]
// compile_error!("esp-idf-hal requires the `driver` ESP-IDF component to be enabled");

// #[cfg(feature = "std")]
// #[allow(unused_imports)]
// #[macro_use]
// extern crate std;

// #[cfg(feature = "alloc")]
// #[allow(unused_imports)]
// #[macro_use]
// extern crate alloc;

// // This is used to create `embedded_hal` compatible error structs
// // that preserve original `EspError`.
// //
// // Example:
// // embedded_hal_error!(I2cError, embedded_hal::i2c::Error, embedded_hal::i2c::ErrorKind)
// #[allow(unused_macros)]
// macro_rules! embedded_hal_error {
//     ($error:ident, $errortrait:ty, $kind:ty) => {
//         #[derive(Debug, Copy, Clone, Eq, PartialEq)]
//         pub struct $error {
//             kind: $kind,
//             cause: esp_idf_sys::EspError,
//         }

//         impl $error {
//             pub fn new(kind: $kind, cause: esp_idf_sys::EspError) -> Self {
//                 Self { kind, cause }
//             }
//             pub fn other(cause: esp_idf_sys::EspError) -> Self {
//                 Self::new(<$kind>::Other, cause)
//             }
//             pub fn cause(&self) -> esp_idf_sys::EspError {
//                 self.cause
//             }
//         }
//         impl From<esp_idf_sys::EspError> for $error {
//             fn from(e: esp_idf_sys::EspError) -> Self {
//                 Self::other(e)
//             }
//         }

//         impl $errortrait for $error {
//             fn kind(&self) -> $kind {
//                 self.kind
//             }
//         }

//         impl core::fmt::Display for $error {
//             fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//                 write!(
//                     f,
//                     "{} {{ kind: {}, cause: {} }}",
//                     stringify!($error),
//                     self.kind,
//                     self.cause()
//                 )
//             }
//         }

//         #[cfg(feature = "std")]
//         impl std::error::Error for $error {}
//     };
// }

// #[macro_export]
// #[allow(unused_macros)]
// macro_rules! into_ref {
//     ($($name:ident),*) => {
//         $(
//             let $name = $name.into_ref();
//         )*
//     }
// }

// #[allow(unused_macros)]
// macro_rules! impl_peripheral_trait {
//     ($type:ident) => {
//         unsafe impl Send for $type {}

//         impl $crate::peripheral::sealed::Sealed for $type {}

//         impl $crate::peripheral::Peripheral for $type {
//             type P = $type;

//             #[inline]
//             unsafe fn clone_unchecked(&mut self) -> Self::P {
//                 $type { ..*self }
//             }
//         }
//     };
// }

// #[allow(unused_macros)]
// macro_rules! impl_peripheral {
//     ($type:ident) => {
//         pub struct $type(::core::marker::PhantomData<*const ()>);

//         impl $type {
//             /// # Safety
//             ///
//             /// Care should be taken not to instantiate this peripheral instance, if it is already instantiated and used elsewhere
//             #[inline(always)]
//             pub unsafe fn new() -> Self {
//                 $type(::core::marker::PhantomData)
//             }
//         }

//         $crate::impl_peripheral_trait!($type);
//     };
// }

// #[allow(unused_imports)]
// pub(crate) use embedded_hal_error;
// #[allow(unused_imports)]
// pub(crate) use impl_peripheral;
// #[allow(unused_imports)]
// pub(crate) use impl_peripheral_trait;