use std::error::Error;

use clap::Parser;
use tokenizers::tokenizer::Tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Optional: add metadata
struct Args {
    #[arg(long)]
    tokenizer_json_path: String,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();
    let embedding_tokenizer = Tokenizer::from_file(args.tokenizer_json_path)?;
    let encoding = embedding_tokenizer.encode("This is a test.", false)?;
    println!("Encoded {} input tokens.", encoding.get_ids().len());
    Ok(())
}
