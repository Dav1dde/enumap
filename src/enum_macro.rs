/// Lightweight macro to automatically implement [`Enum`](crate::Enum).
///
/// The macro automatically implements [`Enum`](crate::Enum), it works only
/// on data less enums and automatically derives `Copy` and `Clone`.
///
/// # Example:
///
/// ```
/// use enumap::EnumMap;
///
/// enumap::enumap! {
///     /// A beautiful fruit, ready to be sold.
///     #[derive(Debug, PartialEq, Eq)]
///     enum Fruit {
///         Orange,
///         Banana,
///         Grape,
///     }
/// }
///
/// // A fruit shop: fruit -> stock.
/// let mut shop = EnumMap::new();
/// shop.insert(Fruit::Orange, 100);
/// shop.insert(Fruit::Banana, 200);
/// shop.insert(Fruit::Grape, 300);
///
/// assert_eq!(
///     shop.into_iter().collect::<Vec<_>>(),
///     vec![(Fruit::Orange, 100), (Fruit::Banana, 200), (Fruit::Grape, 300)],
/// );
///
/// // Oranges out of stock:
/// shop.remove(Fruit::Orange);
///
/// assert_eq!(
///     shop.into_iter().collect::<Vec<_>>(),
///     vec![(Fruit::Banana, 200), (Fruit::Grape, 300)],
/// );
/// # use enumap::Enum;
/// # assert_eq!(Fruit::to_index(Fruit::Orange), 0);
/// # assert_eq!(Fruit::to_index(Fruit::Banana), 1);
/// # assert_eq!(Fruit::to_index(Fruit::Grape), 2);
/// # assert!(matches!(Fruit::from_index(0), Some(Fruit::Orange)));
/// # assert!(matches!(Fruit::from_index(1), Some(Fruit::Banana)));
/// # assert!(matches!(Fruit::from_index(2), Some(Fruit::Grape)));
/// # assert!(matches!(Fruit::from_index(3), None));
/// # assert_eq!(Fruit::LENGTH, 3);
/// ```
#[macro_export]
macro_rules! enumap {
    (
        $(#[$($attr:tt)*])*
        $vis:vis enum $name:ident {
            $(
                $(#[$($vattr:tt)*])*
                $v:ident
            ),* $(,)?
        }
    ) =>{
        $(#[$($attr)*])*
        #[derive(Copy, Clone)]
        $vis enum $name {
            $(
                $(#[$($vattr)*])*
                $v,
            )*
        }

        impl $crate::Enum<{ 0 $(+ $crate::__replace_expr!($v 1))* }> for $name {
            #[allow(unused_variables)]
            fn from_index(index: usize) -> Option<Self> {
                $(
                    if index == 0 { return Some(Self::$v); }
                    let index = index - 1;
                )*
                None
            }

            fn to_index(value: Self) -> usize {
                value as usize
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
