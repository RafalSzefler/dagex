macro_rules! readonly {
    (
        $visible:vis struct $name:ident {
            $($fname:ident : $ftype:ty),* $(,)?
        }
    ) => {
        $visible struct $name {
            $($fname : $ftype),*
        }

        impl $name {

            #[inline(always)]
            $visible fn new($($fname : $ftype),*) -> $name {
                $name { $($fname),* }
            }

            #[inline(always)]
            $($visible fn $fname(&self) -> &$ftype {
                &self.$fname
            })*
        }
    }
}

macro_rules! readonly_derive {
    (
        $visible:vis struct $name:ident {
            $($fname:ident : $ftype:ty),* $(,)?
        }
    ) => {

        #[derive(PartialEq, Eq, Hash, Clone, Debug)]
        $visible struct $name {
            $($fname : $ftype),*
        }

        impl $name {

            #[inline(always)]
            $visible fn new($($fname : $ftype),*) -> $name {
                $name { $($fname),* }
            }

            #[inline(always)]
            $($visible fn $fname(&self) -> &$ftype {
                &self.$fname
            })*
        }
    }
}

pub(crate) use readonly;
pub(crate) use readonly_derive;
