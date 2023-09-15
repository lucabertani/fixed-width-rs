use darling::{ast, util, FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use strum::EnumString;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

// cargo expand --test test_simple
// RUSTFLAGS="-Z macro-backtrace" cargo test
// da eseguire con rust nightly

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(fixed_width), supports(struct_named))]
struct FixedWidthFields {
    ident: Ident,
    data: ast::Data<util::Ignored, FixedWidthField>,
}

//#[darling(default)]
//skip: bool,

#[derive(Debug, FromField)]
#[darling(attributes(fixed_width))]
struct FixedWidthField {
    ident: Option<Ident>,
    ty: Type,
    size: usize,
    #[darling(default = "pad_default")]
    pad: char,
    #[darling(default = "pad_left_default")]
    pad_left: bool,
    #[darling(default = "decimals_default")]
    decimals: usize,
    #[darling(default = "date_format_default")]
    date_format: String,
    #[darling(default = "time_format_default")]
    time_format: String,
    #[darling(default = "date_time_format_default")]
    date_time_format: String,
}

fn pad_default() -> char {
    ' '
}
fn pad_left_default() -> bool {
    true
}
fn decimals_default() -> usize {
    0
}
fn date_format_default() -> String {
    "[year][month][day]".into()
}
fn time_format_default() -> String {
    "[hour padding:none][minute][second]".into()
}
fn date_time_format_default() -> String {
    "[year][month][day] [hour padding:none][minute][second]".into()
}

impl FixedWidthField {
    fn field_name(&self) -> String {
        self.ident()
            .and_then(|i| Some(i.to_string()))
            .unwrap_or(String::new())
    }

    /*fn field_type(&self) -> FieldType {
        if let Type::Path(path) = self.ty() {
            let field_type = &path.path.segments.first().unwrap().ident;
            let field_type_enum = FieldType::from_str(field_type.to_string().as_str()).expect(
                format!("Unable to parse {} into FieldType", field_type.to_string()).as_str(),
            );
            field_type_enum
        } else {
            panic!("Unexpected type: {:?}", self.ty());
        }
    }*/

    fn ident(&self) -> Option<&Ident> {
        self.ident.as_ref()
    }

    /*fn ty(&self) -> &Type {
        &self.ty
    }*/

    fn size(&self) -> usize {
        self.size
    }

    fn pad(&self) -> char {
        self.pad
    }

    fn pad_left(&self) -> bool {
        self.pad_left
    }

    fn decimals(&self) -> usize {
        self.decimals
    }

    fn date_format(&self) -> &str {
        self.date_format.as_ref()
    }

    fn time_format(&self) -> &str {
        self.time_format.as_ref()
    }

    fn date_time_format(&self) -> &str {
        self.date_time_format.as_ref()
    }
}

#[derive(Debug, Clone, Copy, EnumString, strum::Display)]
enum FieldType {
    String,
}

#[proc_macro_derive(FixedWidth, attributes(fixed_width))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let fw: FixedWidthFields = FixedWidthFields::from_derive_input(&input).unwrap();
    //println!("Derive {:#?}", fw);
    //let DeriveInput { ident, .. } = input;
    //println!("Derive {:#?}", input);
    let ident = input.ident;

    let mut fields = Vec::new();

    for field in fw.data.take_struct().unwrap() {
        let field_name = field.field_name();

        let field_name_ts: proc_macro2::TokenStream = field_name.parse().unwrap();
        let size = field.size();
        let pad = field.pad() as u8;
        let pad_left = field.pad_left();
        let decimals = field.decimals();
        let date_format = field.date_format();
        let time_format = field.time_format();
        let date_time_format = field.date_time_format();

        let convert = quote! {
            let mut v = fixed_width::pad(&self.#field_name_ts, #field_name, #size, #pad, #pad_left, #decimals, #date_format, #time_format, #date_time_format)?;
            res.append(&mut v);
        };
        fields.push(convert);
    }

    let output: proc_macro2::TokenStream = quote! {
        impl FixedWidth for #ident {
            fn to_fixed_width_bytes(&self) -> Result<Vec<u8>, fixed_width::error::FixedWidthError> {
                let mut s = String::new();
                let mut res: Vec<u8> = Vec::new();
                #(#fields)*
                Ok(res)
            }
        }
    };

    output.into()
}

// FIXED WIDTH ENUM DERIVE

/*#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct FixedWidthEnumFields {
    ident: Ident,
    data: ast::Data<util::Ignored, FixedWidthEnumField>,
}*/

//#[darling(default)]
//skip: bool,

#[derive(Debug, FromVariant)]
#[darling(from_ident, attributes(hello))]
#[allow(dead_code)]
struct FixedWidthEnumField {
    ident: syn::Ident,
    into: Option<bool>,
    skip: Option<bool>,
    discriminant: Option<syn::Expr>,
    fields: darling::ast::Fields<syn::Type>,
}
impl From<syn::Ident> for FixedWidthEnumField {
    fn from(ident: syn::Ident) -> Self {
        FixedWidthEnumField {
            ident,
            into: Default::default(),
            skip: Default::default(),
            discriminant: None,
            fields: darling::ast::Style::Unit.into(),
        }
    }
}

/*impl FixedWidthEnumField {
    fn field_name(&self) -> String {
        self.ident()
            .and_then(|i| Some(i.to_string()))
            .unwrap_or(String::new())
    }

    /*fn field_type(&self) -> FieldType {
        if let Type::Path(path) = self.ty() {
            let field_type = &path.path.segments.first().unwrap().ident;
            let field_type_enum = FieldType::from_str(field_type.to_string().as_str()).expect(
                format!("Unable to parse {} into FieldType", field_type.to_string()).as_str(),
            );
            field_type_enum
        } else {
            panic!("Unexpected type: {:?}", self.ty());
        }
    }*/

    fn ident(&self) -> Option<&Ident> {
        self.ident.as_ref()
    }

    /*fn ty(&self) -> &Type {
        &self.ty
    }*/
}*/

#[proc_macro_derive(FixedWidthEnum)]
pub fn derive_fixed_width_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    //println!("Derive: {:#?}", input);
    //let fw: FixedWidthEnumFields = FixedWidthEnumFields::from_derive_input(&input).unwrap();
    //println!("FWWW: {:#?}", fw);

    let ident = input.ident;
    if let syn::Data::Enum(enm) = input.data {
        let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
        for variant in enm.variants {
            let field = FixedWidthEnumField::from_variant(&variant).unwrap();
            //println!("A: {:#?}", field);

            let field_name = field.ident.to_string();
            let field_name_ts: proc_macro2::TokenStream = field_name.parse().unwrap();

            let convert = quote! {
                Self::#field_name_ts => String::from(#field_name),
            };

            fields.push(convert);
        }

        let output: proc_macro2::TokenStream = quote! {
            impl FixedWidthEnum for #ident {
                fn key(&self) -> String {
                    match self {
                        #(#fields)*
                    }
                }
            }
        };

        output.into()
    } else {
        panic!("Expect enum, got {:#?}", ident);
    }
}
