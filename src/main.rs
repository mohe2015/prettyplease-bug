pub fn main() {
    let dest_path = std::path::Path::new("lsp.rs");
    let comment = std::iter::repeat("file glob **/*\n").take(10).collect::<String>();
    let token_stream = quote::quote! {
        struct Test {
            #[doc = #comment]
            test: i32
        }
    };
    let file = syn::parse2::<syn::File>(token_stream).unwrap();
    let formatted = prettyplease::unparse(&file);
    std::fs::write(
        &dest_path,
        formatted
    ).unwrap();
}