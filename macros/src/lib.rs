use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn_path::path;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    //let ast = parse_macro_input!(input as DeriveInput);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;
    // let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let breakout_path = path!(breakout::ecs);

    TokenStream::from(quote! {
        impl #breakout_path::component::Component for #struct_name {}

        impl #breakout_path::component::Bundle for #struct_name {
            fn components_ids(
                &self,
                entity: #breakout_path::entity::Entity,
                components: &mut #breakout_path::component::Components,
                storages: &mut #breakout_path::world::Storages,
                ids: &mut impl FnMut(#breakout_path::component::ComponentId)
            ) {
                let component_id = components.init_component::<#struct_name>(storages);

                ids(component_id.clone());
                storages.push_component(
                    entity.clone(),
                    component_id,
                    self.clone()
                );
            }
        }
    })
}
