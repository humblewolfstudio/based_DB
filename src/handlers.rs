pub mod aux_fn;
pub mod create;
pub mod delete;
pub mod find;
pub mod insert;
pub mod peek;
pub mod register;
pub mod update;

pub mod command_handler;

pub use aux_fn::get_data;
pub use create::handle_create;
pub use delete::handle_delete;
pub use find::handle_find;
pub use insert::handle_insert;
pub use peek::handle_peek;
pub use register::handle_register;
pub use update::handle_update;

pub use command_handler::process_command;
pub use command_handler::Command;
