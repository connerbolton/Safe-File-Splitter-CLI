/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{fs::File, io::{BufReader, BufWriter, Write}};
use anyhow::Result;
use crate::Args;
use crate::read::read_line_safely;
use crate::bytes::{size_to_bytes, format_bytes};

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ChunkInfo {
    index: u64,
    rows: u64,
    bytes: u64,
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn print_summary(args: &Args, chunks: &Vec<ChunkInfo>, folder_name: &str) {
    let index_width = chunks.len().to_string().len();

    println!();
    println!("───────────────────────────────────────────────────────────────");
    println!("  Split Summary");
    println!("───────────────────────────────────────────────────────────────");
    println!();
    println!("  Input file : {}", args.file_name.display());
    println!("  Total files: {}", chunks.len());
    println!();
    for chunk in chunks {
        println!("  File {:>width$}: {}_part_{}.csv  ({}, {} rows)",
            chunk.index, folder_name, chunk.index, format_bytes(chunk.bytes), chunk.rows,
            width = index_width
        );
    }
    println!();
    println!("───────────────────────────────────────────────────────────────");
    println!();
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn split_by_size(args: &Args, buf_reader: &mut BufReader<File>) -> Result<()> {

    let size_limit = size_to_bytes(&args.size.as_ref().unwrap())?;
    let mut byte_count: u64 = 0;
    let mut output_index = 1;
    let mut chunk_rows: u64 = 0;
    let mut chunks: Vec<ChunkInfo> = Vec::new();

    let folder_name = args.file_name.file_stem().unwrap().to_string_lossy();
    std::fs::create_dir_all(&*folder_name)?;

    let mut output = File::create(format!("{}/{}_part_{}.csv", folder_name, folder_name, output_index))?;
    let mut buf_writer = BufWriter::new(output);

    let mut header = String::new();
    if args.preserve_header {
        header = read_line_safely(buf_reader);
        buf_writer.write_all(header.as_bytes())?;
    }

    loop {

        let line = read_line_safely(buf_reader);

        if line.is_empty() { break; }

        if (byte_count + line.len() as u64) > size_limit {

            buf_writer.flush()?;
            chunks.push(ChunkInfo { index: output_index, rows: chunk_rows, bytes: byte_count });
            byte_count = 0;
            chunk_rows = 0;
            output_index += 1;
            output = File::create(format!("{}/{}_part_{}.csv", folder_name, folder_name, output_index))?;
            buf_writer = BufWriter::new(output);

            if args.preserve_header {
                buf_writer.write_all(header.as_bytes())?;
            }

        }

        buf_writer.write_all(line.as_bytes())?;
        byte_count += line.len() as u64;
        chunk_rows += 1;

    }

    chunks.push(ChunkInfo { index: output_index, rows: chunk_rows, bytes: byte_count });
    buf_writer.flush()?;
    print_summary(args, &chunks, &folder_name);

    Ok(())

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn split_by_rows(args: &Args, buf_reader: &mut BufReader<File>) -> Result<()> {

    let row_limit = args.rows.unwrap();
    let mut row_count: u64 = 0;
    let mut output_index = 1;
    let mut chunk_bytes: u64 = 0;
    let mut chunks: Vec<ChunkInfo> = Vec::new();

    let folder_name = args.file_name.file_stem().unwrap().to_string_lossy();
    std::fs::create_dir_all(&*folder_name)?;

    let mut output = File::create(format!("{}/{}_part_{}.csv", folder_name, folder_name, output_index))?;
    let mut buf_writer = BufWriter::new(output);

    let mut header = String::new();
    if args.preserve_header {
        header = read_line_safely(buf_reader);
        buf_writer.write_all(header.as_bytes())?;
    }

    loop {

        let line = read_line_safely(buf_reader);

        if line.is_empty() { break; }

        if row_count >= row_limit {

            buf_writer.flush()?;
            chunks.push(ChunkInfo { index: output_index, rows: row_count, bytes: chunk_bytes });
            row_count = 0;
            chunk_bytes = 0;
            output_index += 1;
            output = File::create(format!("{}/{}_part_{}.csv", folder_name, folder_name, output_index))?;
            buf_writer = BufWriter::new(output);

            if args.preserve_header {
                buf_writer.write_all(header.as_bytes())?;
            }

        }

        buf_writer.write_all(line.as_bytes())?;
        row_count += 1;
        chunk_bytes += line.len() as u64;

    }

    chunks.push(ChunkInfo { index: output_index, rows: row_count, bytes: chunk_bytes });
    buf_writer.flush()?;
    print_summary(args, &chunks, &folder_name);

    Ok(())

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////