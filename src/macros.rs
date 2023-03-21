/// Calls a macro for all types up to a given sequence.
macro_rules! _impl_for_all {
        (@step $macro:ident ; ) => {
            $macro!();
        };
        (@step $macro:ident ; $head:ident $(, $items:ident)* ) => {
            $macro!($head $(, $items)*);
            crate::macros::impl_for_all!(@step $macro ; $($items),  *);
        };
        ($macro:ident ; $($items:ident),*) => {
            crate::macros::impl_for_all!(@step $macro ; $($items),*);
        }
    }

pub(crate) use _impl_for_all as impl_for_all;

/// Calls a macro with up to 32 distinct types.
macro_rules! _impl_all {
        ($macro:ident) => {
            crate::macros::impl_for_all!($macro ;
                T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
            );
        }
    }

pub(crate) use _impl_all as impl_all;
