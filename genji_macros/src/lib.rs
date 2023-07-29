use proc_macro::TokenStream;

/// This imports genji into your game. Place on any
/// tokens whatsoever.
/// 
/// Use like this:
/// ```ignore
/// #[genji_init]
/// type State = GameState<()>;
/// ```
/// 
/// Note: disables LSP for the affected token(s).
#[proc_macro_attribute]
pub fn init(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let mainfun: TokenStream = include_str!("insert/main.rs").parse().unwrap();

    item.extend(mainfun);
    item
}
