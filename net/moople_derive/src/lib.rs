use darling::{
    ast::{self, Data},
    util, FromDeriveInput, FromField, FromMeta, ToTokens,
};
use proc_macro2::{Span, TokenStream};
use syn::{
    parse_quote, GenericParam, Generics, Ident, Lifetime, LifetimeDef, Type, TypeParamBound,
};

#[derive(FromMeta, Debug)]
struct Cond {
    pub field: syn::Ident,
    pub cond: syn::Path,
}

impl Cond {
    pub fn cond_self_expr(&self) -> TokenStream {
        let cond_fn = &self.cond;
        let field = &self.field;
        quote::quote! ( #cond_fn( &self.#field ) )
    }

    pub fn cond_expr(&self) -> TokenStream {
        let cond_fn = &self.cond;
        let field = &self.field;
        quote::quote! ( #cond_fn( &#field ) )
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(maple_packet))]
struct MaplePacketField {
    ident: Option<Ident>,
    ty: Type,
    skip_if: Option<Cond>,
    either: Option<Cond>,
}

impl MaplePacketField {
    pub fn get_cond(&self) -> Option<&Cond> {
        self.skip_if.as_ref().or_else(|| self.either.as_ref())
    }

    pub fn encode_expr(&self, field_name: &TokenStream) -> TokenStream {
        if let Some(cond) = self.get_cond() {
            let cond = cond.cond_self_expr();
            quote::quote! ( moople_packet::MapleConditional::encode_packet_cond(&self.#field_name, #cond, pw) )
        } else {
            quote::quote! ( self.#field_name.encode_packet(pw) )
        }
    }

    pub fn packet_len_expr(&self, field_name: &TokenStream) -> TokenStream {
        if let Some(cond) = self.get_cond() {
            let cond = cond.cond_self_expr();
            quote::quote! ( moople_packet::MapleConditional::packet_len_cond(&self.#field_name, #cond) )
        } else {
            quote::quote! ( self.#field_name.packet_len() )
        }
    }

    pub fn decode_expr(&self, var_ident: &Ident) -> TokenStream {
        let ty = &self.ty;
        if let Some(cond) = self.get_cond() {
            let cond = cond.cond_expr();
            quote::quote!( let #var_ident  = <#ty as moople_packet::MapleConditional>::decode_packet_cond(#cond, pr) )
        } else {
            quote::quote!( let #var_ident = <#ty>::decode_packet(pr) )
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(maple_packet), supports(struct_any))]
struct MaplePacket {
    ident: Ident,
    data: ast::Data<util::Ignored, MaplePacketField>,
    generics: syn::Generics,
}

#[proc_macro_derive(MaplePacket, attributes(maple_packet, opcode))]
pub fn maple_packet(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(item as syn::DeriveInput);

    let input = match MaplePacket::from_derive_input(&derive_input) {
        Ok(input) => input,
        Err(err) => return err.write_errors().into(),
    };

    input.to_token_stream().into()
}

fn add_trait_bounds(mut generics: Generics, bound: TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(bound.clone());
        }
    }
    generics
}

fn find_or_add_de_lifetime(generics: &mut Generics) -> &Lifetime {
    let has_lifetime = generics
        .params
        .iter()
        .any(|param| matches!(param, GenericParam::Lifetime(_)));

    if !has_lifetime {
        let lf = Lifetime::new("'a", Span::call_site());
        let lf: GenericParam = LifetimeDef::new(lf).into();
        generics.params.push(lf);
    }

    &generics.lifetimes().next().unwrap().lifetime
}

impl MaplePacket {
    fn fields_with_name(&self) -> impl Iterator<Item = ((Ident, TokenStream), &MaplePacketField)> {
        let Data::Struct(ref fields) = self.data else {
            panic!("Not a struct");
        };

        fields.iter().enumerate().map(|(i, field)| {
            let ident = field
                .ident
                .as_ref()
                .map(|v| (v.clone(), quote::quote!(#v)))
                .unwrap_or_else(|| {
                    let i = syn::Index::from(i);
                    (quote::format_ident!("_{}", i), quote::quote!(#i))
                });

            (ident, field)
        })
    }

    fn gen_encode(&self, token_stream: &mut proc_macro2::TokenStream) -> syn::Result<()> {
        let struct_name = &self.ident;
        let enc_generics = add_trait_bounds(
            self.generics.clone(),
            parse_quote!(moople_packet::EncodePacket),
        );

        let (impl_generics, ty_generics, where_clause) = enc_generics.split_for_impl();

        let struct_enc_fields = self.fields_with_name().map(|((_, field_name), field)| {
            let enc = field.encode_expr(&field_name);
            quote::quote!( #enc?; )
        });

        token_stream.extend(quote::quote!(impl #impl_generics  moople_packet::EncodePacket for #struct_name #ty_generics #where_clause {
            fn encode_packet<B: bytes::BufMut>(&self, pw: &mut moople_packet::MaplePacketWriter<B>) -> moople_packet::NetResult<()> {
                #(#struct_enc_fields)*
                Ok(())
            }
        }));
        Ok(())
    }

    fn gen_decode(&self, token_stream: &mut proc_macro2::TokenStream) -> syn::Result<()> {
        let struct_name = &self.ident;
        let mut dec_generics = self.generics.clone();
        let de_lifetime = find_or_add_de_lifetime(&mut dec_generics).clone();
        let dec_generics = add_trait_bounds(
            dec_generics,
            parse_quote!(moople_packet::proto::DecodePacket<#de_lifetime>),
        );

        let (_, ty_generics, _) = self.generics.split_for_impl();
        let (de_impl_generics, _, de_where_clause) = dec_generics.split_for_impl();

        let dec_var = self.fields_with_name().map(|((var_ident, _), field)| {
            let dec = field.decode_expr(&var_ident);
            quote::quote!( #dec?; )
        });

        let struct_dec_fields = self.fields_with_name().map(|((var_ident, field_name), _)| {
            quote::quote! { #field_name: #var_ident, }
        });

        token_stream.extend(quote::quote!(impl #de_impl_generics  moople_packet::DecodePacket<#de_lifetime> for #struct_name #ty_generics #de_where_clause  {
            fn decode_packet(pr: &mut moople_packet::MaplePacketReader<#de_lifetime>) -> moople_packet::NetResult<Self> {
                #(#dec_var)*
                Ok(#struct_name {
                    #(#struct_dec_fields)*
                })
            }
        }));
        Ok(())
    }

    fn gen_len(&self, token_stream: &mut proc_macro2::TokenStream) -> syn::Result<()> {
        let struct_name = &self.ident;
        let len_generics = add_trait_bounds(
            self.generics.clone(),
            parse_quote!(moople_packet::PacketLen),
        );
        let (len_impl_generics, ty_generics, len_where_clause) = len_generics.split_for_impl();

        let struct_size_hint_fields = self.fields_with_name().map(|(_, field)| {
            let ty = &field.ty;
            quote::quote!(.add(moople_packet::proto::SizeHint(<#ty>::SIZE_HINT)))
        });

        let struct_packet_len_fields = self.fields_with_name().map(|((_, field_name), field)| {
            let len = field.packet_len_expr(&field_name);
            quote::quote!( + #len )
        });

        token_stream.extend(quote::quote! {
            impl #len_impl_generics  moople_packet::PacketLen for #struct_name #ty_generics #len_where_clause  {
                const SIZE_HINT: Option<usize> = moople_packet::proto::SizeHint::zero()#(#struct_size_hint_fields)*.0;

                fn packet_len(&self) -> usize {
                    0 #(#struct_packet_len_fields)*
                }
            }
        });
        Ok(())
    }

    fn gen(&self, tokens: &mut proc_macro2::TokenStream) {
        self.gen_encode(tokens)
            .and_then(|_| self.gen_decode(tokens))
            .and_then(|_| self.gen_len(tokens))
            .unwrap();
    }
}

impl ToTokens for MaplePacket {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.gen(tokens);
    }
}
