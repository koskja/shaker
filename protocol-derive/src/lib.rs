#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol_derive_impl::packet(input)
}
