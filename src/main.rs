mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::SecreteArgs::parse();

    match args.operation_type {
        args::OperationArgs::Encode(args) => commands::encode(args),
        args::OperationArgs::Decode(args) => commands::decode(args),
        args::OperationArgs::Print(args) => commands::print_chunks(args),
        args::OperationArgs::Remove(args) =>commands::remove(args),
    }
}
