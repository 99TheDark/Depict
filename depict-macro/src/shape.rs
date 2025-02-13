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
    pub name: Ident,
    pub fields: Punctuated<ShapeField, token::Comma>,
}

#[derive(Debug)]
pub(crate) struct ShapeField {
    pub name: Ident,
    pub typ: Type,
    pub default: Option<FieldDefault>,
}

#[derive(Debug)]
pub(crate) struct FieldDefault {
    pub value: Expr,
}

impl Parse for Shape {
    fn parse(input: ParseStream) -> Result<Self> {
        let fields;

        let visibility = input.parse()?;
        input.parse::<token::Struct>()?;
        let name = input.parse()?;
        braced!(fields in input);

        Ok(Shape {
            visibility,
            name,
            fields: fields.parse_terminated(ShapeField::parse, token::Comma)?,
        })
    }
}

impl Parse for ShapeField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<token::Colon>()?;

        Ok(ShapeField {
            name,
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
        input.parse::<token::Eq>()?;

        Ok(FieldDefault {
            value: input.parse()?,
        })
    }
}
