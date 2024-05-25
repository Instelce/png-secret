use std::path::PathBuf;

use clap::Subcommand;


#[derive(Subcommand)]
pub enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>
    },

    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },

    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },

    Print {
        file_path: PathBuf,
    }
}
