pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: String,
}

pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

pub enum TypeInfo {
    Pointer {
        to_type: String,
    },
    FnPointer {
        arg_types: Vec<String>,
        ret_type: String,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Array {
        item_type: String,
    },
    None,
}

pub struct Argument {
    pub name: String,
    pub r#type: String,
}

pub struct Type {
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

pub struct Function {
    location: usize,
    size: usize,
    args: Vec<Argument>,
    ret_type: String,
}

pub struct Global {
    location: usize,
    r#type: String,
}