// Não queremos que este módulo apareça na documentação. Módulo é público apenas internamente.
pub(crate) mod contract;
pub mod entry;
pub mod schedule;
pub mod temperature;
pub mod utils;

pub use contract::Contract;
