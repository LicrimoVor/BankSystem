/// `impl_repository!(token: AccountToken, Account (id: Uuid, user_id: Uuid, balance: f64));
#[macro_export]
macro_rules! impl_constructor {
    (token: $token:ident, $type:ident, ($($arg_name:ident: $arg_ty:ty), *)) => {
        struct $token;

        impl $type {
            /// Создание объекта
            /// ```
            /// const token = get_token();
            /// const account = Account::new(token, id, user_id, balance);
            /// ```
            fn new(_token: $token, $($arg_name: $arg_ty), *) -> Self {
                $type {
                    $($arg_name),*
                }
            }
        }

        pub fn get_token() -> $token {
            $token
        }
    };
    (factory: $type:ident, ($($arg_name:ident: $arg_ty:ty), *)) => {
        /// Создание объекта
        /// ```
        /// const account = account::factory::create(id, user_id, balance);
        /// ```
        pub(crate) mod factory {
            use super::*;

            pub fn create($($arg_name: $arg_ty), *) -> $type {
                $type {
                    $($arg_name),*
                }
            }
        }

    };
}
