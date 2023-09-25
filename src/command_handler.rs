use core::fmt;

#[derive(Debug)]
pub enum Command {
    INSERT,
    FIND,
    UPDATE,
}

//Los enums necesitan esto para poder hacer el .to_string()
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::INSERT => write!(f, "INSERT"),
            Command::FIND => write!(f, "FIND"),
            Command::UPDATE => write!(f, "UPDATE"),
        }
    }
}

pub fn process_command(command: &str) -> Result<Command, String> {
    match command.to_lowercase().as_ref() {
        "find" => return Ok(Command::FIND),
        "insert" => return Ok(Command::INSERT),
        "update" => return Ok(Command::UPDATE),
        _ => return Err(String::from("Command doesnt exist")),
    }
}
