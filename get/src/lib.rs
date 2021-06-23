use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, abort_call_site, proc_macro_error, ResultExt};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, DataStruct, DeriveInput, Field, Token, VisPublic};

#[proc_macro_derive(GetCopy)]
#[proc_macro_error]
pub fn get_copy(input: TokenStream) -> TokenStream {
    produce(&syn::parse::<DeriveInput>(input).expect_or_abort("couldn't parse input")).into()
}

fn produce(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields.iter().map(|f| implement_one(f));

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        abort_call_site!("#[derive(Get)] is only defined for structs, not for enums!");
    }
}

fn implement_one(field: &Field) -> TokenStream2 {
    let span = field.span();

    let field_name = field
        .clone()
        .ident
        .unwrap_or_else(|| abort!(field.span(), "Expected the field to have a name"));

    let ty = field.ty.clone();

    let visibility = VisPublic {
        pub_token: Token![pub](Span::call_site()),
    };

    quote_spanned! { span=>
        #[inline(always)]
        #visibility fn #field_name(&self) -> #ty {
            self.#field_name
        }
    }
}
