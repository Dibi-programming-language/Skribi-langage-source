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

// Associated names for the VariableType enum
const STRING_NAME: &str = "skr"; // From "skribi" in dibi
const INTEGER_NAME: &str = "int"; // From "integi" in dibi
const FLOAT_NAME: &str = "dar"; // From "daritmi" in dibi
const BOOLEAN_NAME: &str = "ioi"; // From "ioi" in dibi
const UNSET_NAME: &str = "unset";

/**
 * This is the struct that stores everything about a variable (name, value, scope level, etc.)
 */
#[derive(Debug)]
pub(crate) struct VariableStruct {
    name: String,
    value: VariableType,
    scope_level: u8,
    // 0 is global, this is used to remove variables when exiting it's scope
    is_constant: bool,
    is_set: bool,
    type_name: String,
}

impl VariableStruct {
    /**
     * Change the value of the variable
     */
    pub fn set_value(&mut self, value: VariableType, line: u16) {
        // check if the variable is constant
        if self.is_constant {
            error("Cannot redefine value of constant", line);
        }
        if !self.is_set {
            self.is_set = true
        }

        // check if the variable type are the same
        match value {
            VariableType::String(_) => {
                if &self.type_name != STRING_NAME {
                    error(
                        ("Cannot set ".to_string() + &self.type_name + " to " + STRING_NAME).as_str(),
                        line,
                    );
                }
            }
            VariableType::Integer(_) => {
                if &self.type_name != INTEGER_NAME {
                    error(
                        ("Cannot set ".to_string() + &self.type_name + " to " + INTEGER_NAME).as_str(),
                        line,
                    );
                }
            }
            VariableType::Float(_) => {
                if &self.type_name != FLOAT_NAME {
                    error(
                        ("Cannot set ".to_string() + &self.type_name + " to " + FLOAT_NAME).as_str(),
                        line,
                    );
                }
            }
            VariableType::Boolean(_) => {
                if &self.type_name != BOOLEAN_NAME {
                    error(
                        ("Cannot set ".to_string() + &self.type_name + " to " + BOOLEAN_NAME).as_str(),
                        line,
                    );
                }
            }
            VariableType::Unset => {
                error(("Cannot set a variable to ".to_string() + UNSET_NAME).as_str(), line);
            }
        }

        self.value = value;
    }

    /**
     * Return the value of the variable
     */
    pub fn get_value(&mut self, line: u16) -> &VariableType {
        if !self.is_set {
            error("Variable was never initialized", line)
        }
        &self.value
    }
}

/**
 * This function is used to create a new variable
 */
pub(crate) fn new_variable(
    line: Vec<String>,
    scope_level: u8,
    line_number: u16,
) -> (VariableStruct, String) {

    // Check "pu", "fu" and "ju" keywords
    let mut is_constant = false;
    let mut is_private = false;
    let mut is_global = false;
    let mut modifiers_number = 0;
    let args = line[0..2].to_vec();

    // get the modifiers of the variable (global, private, constant)
    // if there is only 1 modifier, we do not have to check if he is in the first position because the type will be unknown.
    if args.contains(&"pu".to_string()) {
        is_private = true;
        modifiers_number += 1;
    }
    if args.contains(&"fu".to_string()) {
        is_global = true;
        modifiers_number += 1;
    }
    if args.contains(&"ju".to_string()) {
        is_constant = true;
        modifiers_number += 1;
    }
    if is_global && is_private {
        error("Variable cannot be both global and private", line_number);
    }

    // line[0] is : base keyword or type. line[1] is : base keyword, type or name. We can't use the keyword twice, and
    // a keyword name cannot be used as a type. In the same way, a type name cannot be used as a variable name
    if line[0] == line[1] {
        error(
            ("Syntax error: to many \"".to_string()
                + line[0].to_string().as_str()
                + "\" in variable declaration. The problem is one of the following. You cannot use a keyword like \"pu\", \"ju\" or \"fu\" twice. A keyword cannot be used as a type name. A type cannot be used as a variable name. In any case, your line is not allowed.")
                .as_str(),
            line_number,
        );
    }

    // create an empty variable
    let mut var = VariableType::Unset;

    let line_length = line.len() - modifiers_number;

    if line_length < 2 {
        error(
            "Syntax error: variable declaration need at least a type and a name",
            line_number,
        );
    } else if line_length > 3 {
        error(
            "Syntax error: variable declaration can only have a type, a name and a value",
            line_number,
        );
    } else if line_length == 3 {
        // if a value is specified get the type and value of the variable
        match line[modifiers_number].clone().as_str() {
            STRING_NAME => {
                var = VariableType::String(line[modifiers_number + 2].to_string());
            }
            INTEGER_NAME => {
                var = VariableType::Integer(line[modifiers_number + 2].parse::<i32>().unwrap());
            }
            FLOAT_NAME => {
                var = VariableType::Float(line[modifiers_number + 2].parse::<f32>().unwrap());
            }
            BOOLEAN_NAME => {
                var = VariableType::Boolean(line[modifiers_number + 2].parse::<bool>().unwrap());
            }
            "ju" | "fu" | "pu" => {
                error("Unknown variable type. A modifier is used in the position of the type. Consider switching", line_number);
            }
            _ => {
                // Call the error function with "Unknown variable type [variable type]" as argument
                error(
                    ("Unknown variable type ".to_string() + &line[modifiers_number]).as_str(),
                    line_number,
                );
            }
        }
    } else {
        // if no values are specified set a default value for the variable
        match line[modifiers_number].clone().as_str() {
            STRING_NAME => {
                var = VariableType::String("".to_string());
            }
            INTEGER_NAME => {
                var = VariableType::Integer(0);
            }
            FLOAT_NAME => {
                var = VariableType::Float(0.0);
            }
            BOOLEAN_NAME => {
                var = VariableType::Boolean(false);
            }
            "ju" | "fu" | "pu" => {
                error("Unknown variable type. A modifier is used in the position of the type. Consider switching.", line_number);
            }
            _ => {
                error("Unknown variable type", line_number);
            }
        }
    }

    // return the variable
    (
        VariableStruct {
            name: line[modifiers_number + 1].clone(),
            value: var,
            scope_level: if is_global { 0 } else { scope_level },
            is_constant,
            is_set: line_length == 3,
            type_name: line[modifiers_number].clone(),
        },
        line[modifiers_number + 1].clone(),
    )
}
