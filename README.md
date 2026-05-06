# filesplit

A fast, safe CLI tool for splitting large CSV files without breaking rows or corrupting quoted fields.

## The Problem

Naive file splitting is dangerous — it can cut rows in the middle, break quoted CSV records that span multiple lines, and corrupt downstream processing. `filesplit` solves this by reading complete logical records before splitting, handling quoted fields and escaped quotes correctly.

## Installation

Clone the repo and install with Cargo:

```bash
git clone https://github.com/yourusername/filesplit
cd filesplit
cargo install --path .
```

## Usage

Split by file size:
```bash
filesplit -f large.csv --size 1GB
filesplit -f large.csv --size 500MB
filesplit -f large.csv --size 100KB
```

Split by row count:
```bash
filesplit -f large.csv --rows 500000
```

Preserve the header row in every output chunk:
```bash
filesplit -f large.csv --size 1GB --preserve-header
filesplit -f large.csv --rows 500000 --preserve-header
```

## Output

Output files are created in a folder named after the input file in the current directory:
```bash
dataset/
├── dataset_part_1.csv
├── dataset_part_2.csv
└── dataset_part_3.csv
```

A summary report is printed after splitting:
```bash
───────────────────────────────────────────────────────────────
Split Summary
───────────────────────────────────────────────────────────────
Input file : dataset.csv
Total files: 3
File 1: dataset_part_1.csv  (999.35 KB, 415 rows)
File 2: dataset_part_2.csv  (998.84 KB, 482 rows)
File 3: dataset_part_3.csv  (176.87 KB, 64 rows)
───────────────────────────────────────────────────────────────
```
## Flags

| Flag | Description |
|------|-------------|
| `-f` | Input file path |
| `--size` | Max size per chunk (KB, MB, GB) |
| `--rows` | Max rows per chunk |
| `--preserve-header` | Write header to every output chunk |

## How It Works

- Streams the file line by line — never loads the whole file into memory
- Tracks open and closed quoted fields to detect complete records
- Handles escaped double quotes (`""`) inside quoted fields
- Rolls over to a new file only on complete row boundaries#
