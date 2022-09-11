use syn::{parse_quote_spanned, spanned::Spanned, Expr, Stmt, Type};

use crate::attributes::PacketAttribute::{self, *};

pub fn to_preprocess(attribute: &PacketAttribute, field: Expr) -> Option<Stmt> {
    match attribute {
        Vec(data) => {
            let target = &data.target;
            Some(parse_quote_spanned! {field.span()=>
                let #target = ::falcon_packet_core::PacketSizeSeed::size(
                    ::falcon_packet_core::PacketVec::new(0),
                    &#field,
                );
            })
        }
        Bytes(data) => data.target.as_ref().map(|target| {
            parse_quote_spanned! {field.span()=>
                let #target = ::falcon_packet_core::PacketSize::size(
                    &#field,
                );
            }
        }),
        AsRef(data) => data.target.as_ref().map(|target| {
            parse_quote_spanned! {field.span()=>
                let #target = ::falcon_packet_core::PacketSizeSeed::size(
                    ::falcon_packet_core::AsRefU8::default(),
                    &#field,
                );
            }
        }),
        _ => None,
    }
}

pub fn to_end(attribute: &PacketAttribute, field: Expr) -> Expr {
    match attribute {
        String(data) => {
            let len = &data.max_length;
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::PacketString::new(#len),
                    &#field,
                )
            }
        }
        Vec(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::PacketVec::new(0),
                    &#field,
                )
            }
        }
        Array(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::PacketArray::default(),
                    &#field,
                )
            }
        }
        AsRef(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSizeSeed::size(
                    &::falcon_packet_core::AsRefU8::default(),
                    &#field,
                )
            }
        }
        _ => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::PacketSize::size(
                    &#field,
                )
            }
        }
    }
}

pub fn to_tokenstream(attribute: &PacketAttribute, field: Expr, field_ty: &Type) -> Expr {
    match attribute {
        VarI32(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::VarI32::from(#field)
            }
        }
        VarI64(_) => {
            parse_quote_spanned! {field.span()=>
                ::falcon_packet_core::VarI64::from(#field)
            }
        }
        Into(data) => {
            let target = &data.target;
            parse_quote_spanned! {field.span()=>
                <#field_ty as ::std::convert::Into<#target>>::into(::std::clone::Clone::clone(&#field))
            }
        }
        Convert(data) => {
            let target = &data.target;
            parse_quote_spanned! {field.span()=>
                <#field_ty as ::std::convert::Into<#target>>::into(::std::clone::Clone::clone(&#field))
            }
        }
        _ => field,
    }
}
