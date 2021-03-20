use clap::arg_enum;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq)]
    pub enum Priority {
        high,
        mid,
        low,
    }
}

