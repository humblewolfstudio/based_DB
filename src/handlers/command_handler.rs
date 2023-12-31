use core::fmt;

#[derive(Debug)]
pub enum Command {
    INSERT,
    FIND,
    UPDATE,
    CREATE, //For creating databases
    PEEK,
    DELETE,
    REGISTER,
}

//Los enums necesitan esto para poder hacer el .to_string()
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::INSERT => write!(f, "INSERT"),
            Command::FIND => write!(f, "FIND"),
            Command::UPDATE => write!(f, "UPDATE"),
            Command::CREATE => write!(f, "CREATE"),
            Command::PEEK => write!(f, "PEEK"),
            Command::DELETE => write!(f, "DELETE"),
            Command::REGISTER => write!(f, "REGISTER"),
        }
    }
}

pub fn process_command(command: &str) -> Result<Command, String> {
    match command.to_lowercase().as_ref() {
        "find" => return Ok(Command::FIND),
        "insert" => return Ok(Command::INSERT),
        "update" => return Ok(Command::UPDATE),
        "create" => return Ok(Command::CREATE),
        "peek" => return Ok(Command::PEEK),
        "delete" => return Ok(Command::DELETE),
        "register" => return Ok(Command::REGISTER),
        _ => return Err(String::from("Command doesnt exist")),
    }
}
