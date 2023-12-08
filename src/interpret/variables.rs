use skribi_language_source::error;

// the #[derive(Debug)] is used to allow the struct to be printed with the {:?} format, this is NOT a comment

/**
 * This is the variable type (rust) used to store the value of a variable (skribi)
 */
#[derive(Debug)]
pub enum VariableType {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Unset,
}
/**
 * This is the struct that stores everything about a variable (name, value, scope level, etc.)
 */
#[derive(Debug)]
pub(crate) struct VariableStruct {
    name: String,
    value: VariableType,
    scope_level: u8, // 0 is global, this is used to remove variables when exiting it's scope
    is_constant: bool,
    is_set: bool,
}

impl VariableStruct {
    /**
     * Change the value of the variable
     */
    pub fn set_value(&mut self, value: VariableType) {
        // check if the variable is constant
        if self.is_constant {
            error("Cannot redefine value of constant");
        }
        if !self.is_set {
            self.is_set = true
        }

        self.value = value;
    }
    /**
     * Return the value of the variable
     */
    pub fn get_value(&mut self) -> &VariableType {
        if !self.is_set {
            error("Variable was never initialized")
        }
        &self.value
    }
}

/**
 * This function is used to create a new variable
 */
pub(crate) fn new_variable(line: Vec<String>, scope_level: u8) -> (VariableStruct, String) {
    let mut is_constant = false;
    let mut is_private = false;
    let mut is_global = false;
    let mut i = 0;
    let args = line[0..2].to_vec();

    // get the type of the variable (global, private, constant)
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

    // create an empty variable
    let mut var = VariableType::Unset;

    let line_length = line.len() - i;

    if line_length < 2 {
        error("Syntax error")
    } else if line_length > 3 {
        error("Syntax error")
    } else if line_length == 3 {
        // is a value is specified get the type and value of the variable
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
            _ => {
                error("Unknown variable type");
            }
        }
    } else {
        // if no values are specified set a default value for the variable
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
            _ => {
                error("Unknown variable type");
            }
        }
    }

    // return the variable
    (
        VariableStruct {
            name: line[i + 1].clone(),
            value: var,
            scope_level: if is_global { 0 } else { scope_level },
            is_constant,
            is_set: line_length == 3,
        },
        line[i + 1].clone(),
    )
}
