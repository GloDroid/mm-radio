use core::future::Future;
use core::pin::Pin;

use crate::stream::Stream;

/// Trait to represent types that can be created by summing up a stream.
///
/// This trait is used to implement the [`sum`] method on streams. Types which
/// implement the trait can be generated by the [`sum`] method. Like
/// [`FromStream`] this trait should rarely be called directly and instead
/// interacted with through [`Stream::sum`].
///
/// [`sum`]: trait.Sum.html#tymethod.sum
/// [`FromStream`]: trait.FromStream.html
/// [`Stream::sum`]: trait.Stream.html#method.sum
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait Sum<A = Self>: Sized {
    /// Method which takes a stream and generates `Self` from the elements by
    /// "summing up" the items.
    fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        S: Stream<Item = A> + 'a;
}

use crate::stream::stream::StreamExt;
use core::num::Wrapping;
use core::ops::Add;

macro_rules! num_sum {
    ($zero:expr, $($a:ty)*) => ($(
        impl Sum for $a {
            fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Self>+ 'a>>
            where
                S: Stream<Item = $a> + 'a,
            {
                Box::pin(async move { stream.fold($zero, Add::add).await } )
            }
        }
        impl<'a> Sum<&'a $a> for $a {
            fn sum<'b, S>(stream: S) -> Pin<Box<dyn Future<Output = Self> + 'b>>
            where
                S: Stream<Item = &'a $a> + 'b,
            {
                Box::pin(async move { stream.fold($zero, Add::add).await } )
            }
        }
    )*);
}

macro_rules! integer_sum {
    ($($a:ty)*) => (
        num_sum!(0, $($a)*);
        num_sum!(Wrapping(0), $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum {
    ($($a:ty)*) => (
        num_sum!(0.0, $($a)*);
    );
}

integer_sum! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum! { f32 f64 }
