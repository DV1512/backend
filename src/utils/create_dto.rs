macro_rules! create_dto {
    (
        $from:ident,
        $(#[$meta:meta])*
        struct $to:ident {
            $($vis:vis $field:ident: $ty:ty,)*
        }
    ) => {
        $(#[$meta])*
        #[derive(Default, serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub(crate) struct $to {
            pub id: crate::dto::IdDTO,
            $($vis $field: $ty,)*
        }

        impl From<$from> for $to {
            fn from(thing: $from) -> Self {
                $to {
                    id: thing.id.into(),
                    $($field: thing.$field,)*
                }
            }
        }

        impl From<Option<$from>> for $to {
            fn from(thing: Option<$from>) -> Self {
                match thing {
                    Some(thing) => thing.into(),
                    None => $to {
                        ..Default::default()
                    }
                }
            }
        }

        impl<'a> From<&'a $from> for $to {
            fn from(thing: &'a $from) -> Self {
                $to {
                    id: thing.id.clone().into(),
                    $($field: thing.$field.clone(),)*
                }
            }
        }
    };
}

pub(crate) use create_dto;
