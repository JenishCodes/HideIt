use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "hideit", version = "0.1.0", author = "Jenish Padodara")]
pub struct SecreteArgs {
    #[clap(subcommand)]
    pub operation_type: OperationArgs,
}

#[derive(Debug, Subcommand)]
pub enum OperationArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// Path of file
    pub file_name: String,

    /// Chunk type
    pub chunk_type: String,

    /// Message to hide
    pub message: String,

    /// Optional: Output file path
    pub output_file_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// Path of file
    pub file_name: String,
    
    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Path of file
    pub file_name: String,

    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// Path of file
    pub file_name: String,
}
