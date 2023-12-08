use skribi_language_source::error;

#[derive(Debug)]
pub enum VariableType {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub(crate) struct VariableStruct {
    pub(crate) name: String,
    pub(crate) value: VariableType,
    pub(crate) scope_level: u8, // 0 is global, this is used to remove variables when exiting it's scope
    is_constant: bool,
    is_set: bool,
}

impl VariableStruct {
    /**
     * Create a new variable
     *
     * # Arguments
     *
     * * `name` - The name of the variable
     * * `value` - The value of the variable
     * * `scope_level` - The scope level of the variable
     * * `is_constant` - If the variable is constant
     */
    pub fn new(
        name: String,
        value: VariableType,
        scope_level: u8,
        is_constant: bool,
        is_set: bool,
    ) -> VariableStruct {
        VariableStruct {
            name,
            value,
            scope_level,
            is_constant,
            is_set,
        }
    }
    pub fn set_value(&mut self, value: VariableType) {
        if self.is_constant {
            error("Cannot redefine value of constant");
        }
        if !self.is_set {
            self.is_set = true
        }
        self.value = value;
    }
    pub fn get_value(&mut self) -> &VariableType {
        if !self.is_set {
            error("Variable not initialized")
        }
        &self.value
    }
}

pub(crate) fn new_variable(line: Vec<String>, scope_level: u8) -> VariableStruct {
    let mut is_constant = false;
    let mut is_private = false;
    let mut is_global = false;
    let mut i = 0;
    let args = line[0..2].to_vec();

    if args.contains(&"pu".to_string()) {
        is_private = true;
        i += 1;
    }
    if args.contains(&"fu".to_string()) {
        is_global = true;
        i += 1;
    }
    if args.contains(&"ju".to_string()) {
        is_constant = true;
        i += 1;
    }

    if is_global && is_private {
        error("Variable cannot be both global and private");
    }
    if line[0] == line[1] {
        error(
            ("Syntax error: to many ".to_string()
                + line[0].to_string().as_str()
                + " in variable declaration")
                .as_str(),
        );
    }

    let mut var = VariableType::Null;

    let line_length = line.len() - i;

    if line_length < 2 {
        error("Syntax error")
    } else if line_length > 3 {
        error("Syntax error")
    } else if line_length == 3 {
        match line[i].clone().as_str() {
            "string" => {
                var = VariableType::String(line[i + 2].to_string());
            }
            "int" => {
                var = VariableType::Integer(line[i + 2].parse::<i32>().unwrap());
            }
            "float" => {
                var = VariableType::Float(line[i + 2].parse::<f32>().unwrap());
            }
            "bool" => {
                var = VariableType::Boolean(line[i + 2].parse::<bool>().unwrap());
            }
            "null" => error("Syntax error"),
            _ => {
                error("Unknown variable type");
            }
        }
    } else {
        match line[i].clone().as_str() {
            "string" => {
                var = VariableType::String("".to_string());
            }
            "int" => {
                var = VariableType::Integer(0);
            }
            "float" => {
                var = VariableType::Float(0.0);
            }
            "bool" => {
                var = VariableType::Boolean(false);
            }
            "null" => {
                if !is_constant {
                    error("Null type cannot be mutable")
                }
                var = VariableType::Null;
            }
            _ => {
                error("Unknown variable type");
            }
        }
    }

    VariableStruct::new(
        line[i + 1].clone(),
        var,
        if is_global { 0 } else { scope_level },
        is_constant,
        line[i].clone().as_str() == "null" || line_length == 3,
    )
}
