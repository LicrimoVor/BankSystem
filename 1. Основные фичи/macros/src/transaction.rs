use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Реализация макроса `#[derive(Transaction)]`
pub fn transaction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut kind = "deposit";

    for attr in &input.attrs {
        if attr.path().is_ident("transaction") {
            // Разбираем атрибут как Meta
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
                let val = meta.value();
                if val == "withdraw" {
                    kind = "withdraw";
                } else if val == "transfer" {
                    kind = "transfer";
                }
            }
        }
    }

    let body = match kind {
        "deposit" => quote! {
            *storage.accounts.entry(self.account.clone()).or_insert(0) += self.amount;
        },
        "withdraw" => quote! {
            let bal = storage.accounts.entry(self.account.clone()).or_insert(0);
            if *bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *bal -= self.amount;
        },
        "transfer" => quote! {
            let from_bal = storage.accounts.entry(self.from.clone()).or_insert(0);
            if *from_bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *from_bal -= self.amount;
            *storage.accounts.entry(self.to.clone()).or_insert(0) += self.amount;
        },
        _ => panic!("Unknown transaction kind"),
    };

    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
                #body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
