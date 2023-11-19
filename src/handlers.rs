pub mod aux_fn;
pub mod create;
pub mod find;
pub mod insert;
pub mod peek;
pub mod update;

pub use aux_fn::get_data;
pub use create::handle_create;
pub use find::handle_find;
pub use insert::handle_insert;
pub use peek::handle_peek;
pub use update::handle_update;
