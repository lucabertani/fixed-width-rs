pub struct FieldConfig {
    field_name: String,
    size: usize,
    pad: u8,
    pad_left: bool,
    decimals: usize,
    add_sign: bool,
    date_format: String, //TODO require field when a Date is passed
    time_format: String,
    date_time_format: String,
}

impl FieldConfig {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        field_name: &str,
        size: usize,
        pad: u8,
        pad_left: bool,
        decimals: usize,
        add_sign: bool,
        date_format: &str,
        time_format: &str,
        date_time_format: &str,
    ) -> FieldConfig {
        FieldConfig {
            field_name: field_name.to_string(),
            size,
            pad,
            pad_left,
            decimals,
            add_sign,
            date_format: date_format.to_string(),
            time_format: time_format.to_string(),
            date_time_format: date_time_format.to_string(),
        }
    }

    pub fn field_name(&self) -> &str {
        self.field_name.as_ref()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn pad(&self) -> u8 {
        self.pad
    }

    pub fn pad_left(&self) -> bool {
        self.pad_left
    }

    pub fn decimals(&self) -> usize {
        self.decimals
    }

    pub fn date_format(&self) -> &str {
        self.date_format.as_ref()
    }

    pub fn time_format(&self) -> &str {
        self.time_format.as_ref()
    }

    pub fn date_time_format(&self) -> &str {
        self.date_time_format.as_ref()
    }

    pub fn add_sign(&self) -> bool {
        self.add_sign
    }
}
