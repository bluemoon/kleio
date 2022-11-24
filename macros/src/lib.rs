use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::DeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(kairos), forward_attrs(allow, doc, cfg))]
struct Options {
  table_name: Option<String>,
  gql_name: Option<String>,
}

#[proc_macro_error]
#[proc_macro_derive(VersionTable, attributes(kairos))]
pub fn derive(input: TokenStream) -> TokenStream {
  // Validate that this is compilable
  let input = syn::parse_macro_input!(input);
  let opts = Options::from_derive_input(&input).expect("Wrong options");
  let DeriveInput { ident, data, .. } = input;

  let struct_data = match data {
    syn::Data::Struct(d) => d,
    syn::Data::Enum(_) => abort!(ident, "cannot be used on an enum"),
    syn::Data::Union(_) => abort!(ident, "cannot be used on a union"),
  };

  let fields = struct_data.fields.iter();

  let version_ident_str = format!("{}Version", ident);
  let table_name = opts.table_name.unwrap_or_else(|| String::from("version"));

  let output = quote! {
    pub mod version {
      use super::*;
      use sea_orm::prelude::*;

      #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        DeriveEntityModel,
        async_graphql::SimpleObject,
        serde::Serialize,
        serde::Deserialize,
      )]
      #[sea_orm(table_name = #table_name)]
      #[graphql(complex, name = #version_ident_str)]
      pub struct Model {
        pub transaction_id: String,
        #(#fields),*
      }

      impl ActiveModelBehavior for ActiveModel {}

      #[async_graphql::ComplexObject]
      impl Model {}
    }
  };

  println!("output: {}", output);
  output.into()
}
