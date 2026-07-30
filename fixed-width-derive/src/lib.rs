use darling::{ast, util, FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use strum::EnumString;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

// cargo expand --test test_simple
// RUSTFLAGS="-Z macro-backtrace" cargo test
// execute with rust nightly

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(fixed_width), supports(struct_named))]
struct FixedWidthFields {
    //ident: Ident,
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
    #[darling(default)]
    pad_if_none: Option<char>,
    #[darling(default = "pad_left_default")]
    pad_left: bool,
    #[darling(default = "decimals_default")]
    decimals: usize,
    #[darling(default = "add_sign_default")]
    add_sign: bool,
    #[darling(default = "date_format_default")]
    date_format: String,
    #[darling(default = "time_format_default")]
    time_format: String,
    #[darling(default = "date_time_format_default")]
    date_time_format: String,
    /// When set, this field always writes the constant value and, on read,
    /// skips the bytes from input (tolerating their absence) and assigns
    /// the constant value to the field. Useful for line-terminator fields
    /// like "\r\n" that get stripped by line readers.
    #[darling(default)]
    constant: Option<String>,
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
fn add_sign_default() -> bool {
    false
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

    fn pad_if_none(&self) -> char {
        self.pad_if_none.unwrap_or(self.pad)
    }

    fn pad_left(&self) -> bool {
        self.pad_left
    }

    fn decimals(&self) -> usize {
        self.decimals
    }

    fn add_sign(&self) -> bool {
        self.add_sign
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

    fn constant(&self) -> Option<&str> {
        self.constant.as_deref()
    }
}

fn get_last_type_segment(ty: &Type) -> String {
    if let Type::Path(p) = ty {
        p.path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

fn get_type_path_segments(ty: &Type) -> Vec<String> {
    if let Type::Path(p) = ty {
        p.path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn get_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(p) = ty {
        if let Some(seg) = p.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                    return Some(inner);
                }
            }
        }
    }
    None
}

fn generate_value_parse(
    field_name: &str,
    ty: &Type,
    type_name: &str,
    pad: u8,
    pad_left: bool,
    decimals: usize,
    add_sign: bool,
    date_format: &str,
    time_format: &str,
    date_time_format: &str,
    path_segments: &[String],
) -> proc_macro2::TokenStream {
    match type_name {
        "String" => {
            quote! {
                String::from_utf8(unpadded.to_vec())
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' as UTF-8 string: {}", #field_name, e)
                    ))?
            }
        }
        "i16" | "i32" | "i64" | "u16" | "u32" | "u64" => {
            quote! {
                {
                    let s = std::str::from_utf8(unpadded)
                        .map_err(|e| fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                        ))?;
                    s.parse::<#ty>()
                        .map_err(|e| fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' value '{}': {}", #field_name, s, e)
                        ))?
                }
            }
        }
        "f32" => generate_float_parse(field_name, decimals, add_sign, quote! { f32 }),
        "f64" => generate_float_parse(field_name, decimals, add_sign, quote! { f64 }),
        "bool" => {
            quote! {
                {
                    let s = std::str::from_utf8(unpadded)
                        .map_err(|e| fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                        ))?;
                    match s.trim() {
                        "1" | "true" => true,
                        "0" | "false" => false,
                        other => return Err(fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' as bool: '{}'", #field_name, other)
                        )),
                    }
                }
            }
        }
        "BigDecimal" => generate_bigdecimal_parse(field_name, decimals, add_sign),
        "Date" => {
            if path_segments.iter().any(|s| s == "chrono") || type_name == "NaiveDate" {
                generate_chrono_date_parse(field_name, date_format)
            } else {
                generate_time_date_parse(field_name, date_format)
            }
        }
        "NaiveDate" => generate_chrono_date_parse(field_name, date_format),
        "Time" if path_segments.iter().any(|s| s == "chrono") => {
            generate_chrono_time_parse(field_name, time_format)
        }
        "NaiveTime" => generate_chrono_time_parse(field_name, time_format),
        "Time" => generate_time_time_parse(field_name, time_format),
        "PrimitiveDateTime" => generate_time_datetime_parse(field_name, date_time_format),
        "NaiveDateTime" => generate_chrono_datetime_parse(field_name, date_time_format),
        _ => {
            // Assume the type implements FixedWidthEnum
            quote! {
                {
                    let s = std::str::from_utf8(unpadded)
                        .map_err(|e| fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                        ))?;
                    <#ty as fixed_width::FixedWidthEnum>::from_key(s)?
                }
            }
        }
    }
}

fn generate_float_parse(
    field_name: &str,
    decimals: usize,
    add_sign: bool,
    float_ty: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if decimals > 0 {
        let actual_decimals = if add_sign { decimals - 1 } else { decimals };
        quote! {
            {
                let s = std::str::from_utf8(unpadded)
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                    ))?;
                let (num_str, sign) = if #add_sign && !s.is_empty() {
                    let sign_char = s.as_bytes()[s.len() - 1];
                    let num = &s[..s.len() - 1];
                    let sign: #float_ty = if sign_char == b'-' { -1.0 } else { 1.0 };
                    (num, sign)
                } else {
                    (s, 1.0 as #float_ty)
                };
                if num_str.is_empty() {
                    0.0 as #float_ty
                } else {
                    let dec_digits = #actual_decimals;
                    if dec_digits > 0 && num_str.len() > dec_digits {
                        let int_part = &num_str[..num_str.len() - dec_digits];
                        let dec_part = &num_str[num_str.len() - dec_digits..];
                        let combined = format!("{}.{}", int_part, dec_part);
                        let val: #float_ty = combined.parse()
                            .map_err(|e: <#float_ty as std::str::FromStr>::Err| fixed_width::error::FixedWidthError::new(
                                format!("Failed to parse field '{}' value '{}': {}", #field_name, combined, e)
                            ))?;
                        val * sign
                    } else {
                        let val: #float_ty = num_str.parse()
                            .map_err(|e: <#float_ty as std::str::FromStr>::Err| fixed_width::error::FixedWidthError::new(
                                format!("Failed to parse field '{}' value '{}': {}", #field_name, num_str, e)
                            ))?;
                        val * sign
                    }
                }
            }
        }
    } else {
        quote! {
            {
                let s = std::str::from_utf8(unpadded)
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                    ))?;
                s.parse::<#float_ty>()
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' value '{}': {}", #field_name, s, e)
                    ))?
            }
        }
    }
}

fn generate_bigdecimal_parse(
    field_name: &str,
    decimals: usize,
    add_sign: bool,
) -> proc_macro2::TokenStream {
    if decimals > 0 {
        let actual_decimals = if add_sign { decimals - 1 } else { decimals };
        quote! {
            {
                let s = std::str::from_utf8(unpadded)
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                    ))?;
                let (num_str, negative) = if #add_sign && !s.is_empty() {
                    let sign_char = s.as_bytes()[s.len() - 1];
                    let num = &s[..s.len() - 1];
                    (num, sign_char == b'-')
                } else {
                    (s, false)
                };
                if num_str.is_empty() {
                    bigdecimal::BigDecimal::from(0)
                } else {
                    let dec_digits = #actual_decimals;
                    let combined = if dec_digits > 0 && num_str.len() > dec_digits {
                        let int_part = &num_str[..num_str.len() - dec_digits];
                        let dec_part = &num_str[num_str.len() - dec_digits..];
                        format!("{}.{}", int_part, dec_part)
                    } else {
                        num_str.to_string()
                    };
                    let val: bigdecimal::BigDecimal = combined.parse()
                        .map_err(|e: bigdecimal::ParseBigDecimalError| fixed_width::error::FixedWidthError::new(
                            format!("Failed to parse field '{}' value '{}': {}", #field_name, combined, e)
                        ))?;
                    if negative { -val } else { val }
                }
            }
        }
    } else {
        quote! {
            {
                let s = std::str::from_utf8(unpadded)
                    .map_err(|e| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                    ))?;
                let (num_str, negative) = if #add_sign && !s.is_empty() {
                    let sign_char = s.as_bytes()[s.len() - 1];
                    let num = &s[..s.len() - 1];
                    (num, sign_char == b'-')
                } else {
                    (s, false)
                };
                let val: bigdecimal::BigDecimal = num_str.parse()
                    .map_err(|e: bigdecimal::ParseBigDecimalError| fixed_width::error::FixedWidthError::new(
                        format!("Failed to parse field '{}' value '{}': {}", #field_name, num_str, e)
                    ))?;
                if negative { -val } else { val }
            }
        }
    }
}

fn generate_time_date_parse(field_name: &str, date_format: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            let format = time::format_description::parse(#date_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Invalid date format for field '{}': {}", #field_name, e)
                ))?;
            time::Date::parse(s, &format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as Date: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_time_time_parse(field_name: &str, time_format: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            let format = time::format_description::parse(#time_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Invalid time format for field '{}': {}", #field_name, e)
                ))?;
            time::Time::parse(s, &format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as Time: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_time_datetime_parse(
    field_name: &str,
    date_time_format: &str,
) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            let format = time::format_description::parse(#date_time_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Invalid datetime format for field '{}': {}", #field_name, e)
                ))?;
            time::PrimitiveDateTime::parse(s, &format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as PrimitiveDateTime: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_chrono_date_parse(field_name: &str, date_format: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            chrono::NaiveDate::parse_from_str(s, #date_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as NaiveDate: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_chrono_time_parse(field_name: &str, time_format: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            chrono::NaiveTime::parse_from_str(s, #time_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as NaiveTime: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_chrono_datetime_parse(
    field_name: &str,
    date_time_format: &str,
) -> proc_macro2::TokenStream {
    quote! {
        {
            let s = std::str::from_utf8(unpadded)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' as UTF-8: {}", #field_name, e)
                ))?;
            chrono::NaiveDateTime::parse_from_str(s, #date_time_format)
                .map_err(|e| fixed_width::error::FixedWidthError::new(
                    format!("Failed to parse field '{}' value '{}' as NaiveDateTime: {}", #field_name, s, e)
                ))?
        }
    }
}

fn generate_field_parse(field: &FixedWidthField) -> proc_macro2::TokenStream {
    let field_name = field.field_name();
    let field_name_ts: proc_macro2::TokenStream = field_name.parse().unwrap();
    let size = field.size();
    let pad = field.pad() as u8;
    let pad_if_none = field.pad_if_none() as u8;
    let pad_left = field.pad_left();
    let decimals = field.decimals();
    let add_sign = field.add_sign();
    let date_format = field.date_format();
    let time_format = field.time_format();
    let date_time_format = field.date_time_format();
    let ty = &field.ty;

    let type_name = get_last_type_segment(ty);
    let path_segments = get_type_path_segments(ty);

    match type_name.as_str() {
        "Vec" => {
            let inner_ty = get_inner_type(ty).expect("Vec must have inner type");
            if pad_left {
                // pad_left=true: content is right-aligned, padding at the beginning
                quote! {
                    let field_bytes = &bytes[offset..offset + #size];
                    let element_size = <#inner_ty as fixed_width::FixedWidth>::fixed_width_size();
                    let max_elements = #size / element_size;
                    let aligned_start = #size - max_elements * element_size;
                    let mut #field_name_ts = Vec::new();
                    let mut _found_content = false;
                    for _i in 0..max_elements {
                        let start = aligned_start + _i * element_size;
                        let chunk = &field_bytes[start..start + element_size];
                        if !_found_content && chunk.iter().all(|&b| b == #pad) {
                            continue;
                        }
                        _found_content = true;
                        #field_name_ts.push(<#inner_ty as fixed_width::FixedWidth>::from_fixed_width_bytes(chunk)?);
                    }
                    offset += #size;
                }
            } else {
                // pad_left=false: content is left-aligned, padding at the end
                quote! {
                    let field_bytes = &bytes[offset..offset + #size];
                    let element_size = <#inner_ty as fixed_width::FixedWidth>::fixed_width_size();
                    let max_elements = #size / element_size;
                    let mut #field_name_ts = Vec::new();
                    for _i in 0..max_elements {
                        let start = _i * element_size;
                        let chunk = &field_bytes[start..start + element_size];
                        if chunk.iter().all(|&b| b == #pad) {
                            break;
                        }
                        #field_name_ts.push(<#inner_ty as fixed_width::FixedWidth>::from_fixed_width_bytes(chunk)?);
                    }
                    offset += #size;
                }
            }
        }
        "Option" => {
            let inner_ty = get_inner_type(ty).expect("Option must have inner type");
            let inner_type_name = get_last_type_segment(inner_ty);
            let inner_path_segments = get_type_path_segments(inner_ty);
            let inner_parse = generate_value_parse(
                &field_name,
                inner_ty,
                &inner_type_name,
                pad,
                pad_left,
                decimals,
                add_sign,
                date_format,
                time_format,
                date_time_format,
                &inner_path_segments,
            );
            quote! {
                let field_bytes = &bytes[offset..offset + #size];
                let #field_name_ts = if field_bytes.iter().all(|&b| b == #pad || b == #pad_if_none) {
                    None
                } else {
                    let unpadded = fixed_width::unpad(field_bytes, #pad, #pad_left);
                    Some(#inner_parse)
                };
                offset += #size;
            }
        }
        _ => {
            let value_parse = generate_value_parse(
                &field_name,
                ty,
                &type_name,
                pad,
                pad_left,
                decimals,
                add_sign,
                date_format,
                time_format,
                date_time_format,
                &path_segments,
            );
            quote! {
                let field_bytes = &bytes[offset..offset + #size];
                let unpadded = fixed_width::unpad(field_bytes, #pad, #pad_left);
                let #field_name_ts = #value_parse;
                offset += #size;
            }
        }
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
    let mut parse_fields = Vec::new();
    let mut field_names = Vec::new();
    let mut size_tokens = Vec::new();
    let mut constant_sizes: Vec<usize> = Vec::new();

    for field in fw.data.take_struct().unwrap() {
        let field_name = field.field_name();

        let field_name_ts: proc_macro2::TokenStream = field_name.parse().unwrap();
        let size = field.size();
        let pad = field.pad() as u8;
        let pad_if_none = field.pad_if_none() as u8;
        let pad_left = field.pad_left();
        let decimals = field.decimals();
        let add_sign = field.add_sign();
        let date_format = field.date_format();
        let time_format = field.time_format();
        let date_time_format = field.date_time_format();
        let constant = field.constant();

        if let Some(const_val) = constant {
            // Validate constant length at compile time
            if const_val.len() > size {
                panic!(
                    "constant value '{}' for field '{}' is {} bytes but field size is {}",
                    const_val,
                    field_name,
                    const_val.len(),
                    size
                );
            }
        }

        let convert = if let Some(const_val) = constant {
            let const_len = const_val.len();
            quote! {
                {
                    let mut v: Vec<u8> = Vec::new();
                    v.extend_from_slice(#const_val.as_bytes());
                    // pad to size
                    let padding_needed = #size - #const_len;
                    for _ in 0..padding_needed {
                        if #pad_left {
                            v.insert(0, #pad);
                        } else {
                            v.push(#pad);
                        }
                    }
                    res.append(&mut v);
                }
            }
        } else {
            quote! {
                let mut v = fixed_width::pad(&self.#field_name_ts, #field_name, #size, #pad, #pad_if_none, #pad_left, #decimals, #add_sign, #date_format, #time_format, #date_time_format)?;
                res.append(&mut v);
            }
        };
        fields.push(convert);

        // Parse code
        let parse_code = if let Some(const_val) = constant {
            let const_str = const_val.to_string();
            constant_sizes.push(size);

            // Check if field type is Option<String>
            let type_name = get_last_type_segment(&field.ty);
            let is_option_string = type_name == "Option"
                && get_inner_type(&field.ty)
                    .map(|inner| get_last_type_segment(inner) == "String")
                    .unwrap_or(false);

            if is_option_string {
                quote! {
                    let #field_name_ts = if offset + #size <= bytes.len() {
                        offset += #size;
                        Some(#const_str.to_string())
                    } else {
                        Some(#const_str.to_string())
                    };
                }
            } else {
                quote! {
                    let #field_name_ts = if offset + #size <= bytes.len() {
                        offset += #size;
                        #const_str.to_string()
                    } else {
                        #const_str.to_string()
                    };
                }
            }
        } else {
            generate_field_parse(&field)
        };
        parse_fields.push(parse_code);

        field_names.push(field_name_ts);
        size_tokens.push(quote! { #size });
    }

    // Sum of sizes of constant fields — these may be missing from input
    let constant_total: usize = constant_sizes.iter().sum();
    let constant_total_token: proc_macro2::TokenStream =
        syn::parse_str(&constant_total.to_string()).unwrap();

    let output: proc_macro2::TokenStream = quote! {
        impl FixedWidth for #ident {
            fn to_fixed_width_bytes(&self) -> Result<Vec<u8>, fixed_width::error::FixedWidthError> {
                let mut s = String::new();
                let mut res: Vec<u8> = Vec::new();
                #(#fields)*
                Ok(res)
            }

            fn fixed_width_size() -> usize {
                0 #(+ #size_tokens)*
            }

            fn from_fixed_width_bytes(bytes: &[u8]) -> Result<Self, fixed_width::error::FixedWidthError> {
                let expected_size = 0 #(+ #size_tokens)*;
                let min_expected_size = expected_size - #constant_total_token;
                if bytes.len() < min_expected_size {
                    return Err(fixed_width::error::FixedWidthError::new(
                        format!("Expected at least {} bytes, got {}", min_expected_size, bytes.len())
                    ));
                }
                let mut offset: usize = 0;
                #(#parse_fields)*
                Ok(Self {
                    #(#field_names),*
                })
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
        let mut key_fields: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut from_key_fields: Vec<proc_macro2::TokenStream> = Vec::new();
        for variant in enm.variants {
            let field = FixedWidthEnumField::from_variant(&variant).unwrap();

            let field_name = field.ident.to_string();
            let field_name_ts: proc_macro2::TokenStream = field_name.parse().unwrap();

            key_fields.push(quote! {
                Self::#field_name_ts => String::from(#field_name),
            });

            from_key_fields.push(quote! {
                #field_name => Ok(Self::#field_name_ts),
            });
        }

        let output: proc_macro2::TokenStream = quote! {
            impl FixedWidthEnum for #ident {
                fn key(&self) -> String {
                    match self {
                        #(#key_fields)*
                    }
                }

                fn from_key(key: &str) -> Result<Self, fixed_width::error::FixedWidthError> {
                    match key {
                        #(#from_key_fields)*
                        _ => Err(fixed_width::error::FixedWidthError::new(
                            format!("Unknown enum key '{}' for type '{}'", key, stringify!(#ident))
                        )),
                    }
                }
            }
        };

        output.into()
    } else {
        panic!("Expect enum, got {:#?}", ident);
    }
}
