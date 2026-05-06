/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

mod split;
mod read;
mod bytes;
use std::{fs::File, io::BufReader, path::PathBuf};
use clap::Parser;
use anyhow::Result;
use crate::split::split_by_size;
use crate::split::split_by_rows;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {

    /// Name of the CSV file you want to split
    #[arg(short = 'f')]
    file_name: PathBuf,

    /// Desired memory size of output file
    #[arg(long, conflicts_with="rows")]
    size: Option<String>,

    /// Desired row size of output file
    #[arg(long, conflicts_with="size")]
    rows: Option<u64>,

    /// Preserve header of CSV file
    #[arg(long)]
    preserve_header: bool

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {

    let args = Args::parse();
    let input_file = File::open(&args.file_name)?;
    let mut buf_reader = BufReader::new(input_file);

    if args.size.is_none() && args.rows.is_none() {
        return Err(anyhow::anyhow!("You must provide either --size or --rows"));
    } 

    else if args.size.is_some() {        
        split_by_size(&args, &mut buf_reader)?;
    }

    else if args.rows.is_some() {
        split_by_rows(&args, &mut buf_reader)?;
    }

    Ok(())

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////