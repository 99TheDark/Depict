extern crate proc_macro;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self},
    Expr, Ident, Result, Type, Visibility,
};

#[derive(Debug)]
pub(crate) struct Shape {
    pub visibility: Visibility,
    pub struct_tok: token::Struct,
    pub name: Ident,
    pub brace_tok: token::Brace,
    pub fields: Punctuated<ShapeField, token::Comma>,
}

#[derive(Debug)]
pub(crate) struct ShapeField {
    pub name: Ident,
    pub colon_tok: token::Colon,
    pub typ: Type,
    pub default: Option<FieldDefault>,
}

#[derive(Debug)]
pub(crate) struct FieldDefault {
    pub eq_tok: token::Eq,
    pub value: Expr,
}

impl Parse for Shape {
    fn parse(input: ParseStream) -> Result<Self> {
        let fields;
        Ok(Shape {
            visibility: input.parse()?,
            struct_tok: input.parse()?,
            name: input.parse()?,
            brace_tok: braced!(fields in input),
            fields: fields.parse_terminated(ShapeField::parse, token::Comma)?,
        })
    }
}

impl Parse for ShapeField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ShapeField {
            name: input.parse()?,
            colon_tok: input.parse()?,
            typ: input.parse()?,
            default: if input.peek(token::Eq) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for FieldDefault {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(FieldDefault {
            eq_tok: input.parse()?,
            value: input.parse()?,
        })
    }
}
