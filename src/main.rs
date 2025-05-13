use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::*;
use placecare::io::{RecordDesc, SearchResult};
use placecare::place_desc::SeqDesc;
use placecare::place_search;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Search {
            input,
            input_seq,
            print,
            write,
            outfile,
        } => {
            let mut res = vec![];

            if let Some(input) = input {
                println!("Input file: {}", input);
                let inputs = RecordDesc::from_file(&input);
                res.extend(
                    place_search::Search::search_elements(&inputs)
                        .expect("Error: Failed to search elements"),
                );
            } else if let Some(input_seq) = input_seq {
                println!("Input sequence: {}", input_seq);
                let inputs = RecordDesc::new("GhInput", &input_seq);
                res.extend(
                    place_search::Search::search_elements_single_seq(&inputs)
                        .expect("Error: Failed to search elements"),
                );
            }

            let output = print_search(res);
            if print {
                println!("{}", output);
            } else if write {
                let path = PathBuf::from(&outfile);
                if let Err(e) = write_content(path, output) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("done at: {}", outfile);
                }
            }
        }
        Commands::Query {
            input,
            input_text,
            id,
            ac,
            print,
            write,
            outfile,
        } => {
            let mut inputs = vec![];
            let mut res = vec![];

            if let Some(input) = input {
                println!("Input file: {}", input);
                let input = fs::read_to_string(input).expect("Error: Failed to read file");
                let input = input.lines().collect::<Vec<_>>();
                for x in input {
                    inputs.push(x.to_string());
                }
            } else if let Some(input_seq) = input_text {
                println!("Input text: {}", input_seq);
                inputs.push(input_seq);
            }
            let inputs = inputs.iter().map(|x| x.as_str()).collect::<Vec<_>>();

            if id {
                res.extend(place_search::Search::query_elements_by_id(&inputs));
            } else if ac {
                res.extend(place_search::Search::query_elements_by_ac(&inputs));
            }

            let output = print_query(res);
            if print {
                println!("{}", output);
            } else if write {
                let path = PathBuf::from(&outfile);
                if let Err(e) = write_content(path, output) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("done at: {}", outfile);
                }
                println!("Output method: write to {}", outfile);
            }
        }
    }
}

fn print_search(res: Vec<SearchResult>) -> String {
    let mut output = String::new();
    for x in res.clone() {
        output.push_str(&format!("ID: {}\t", x.id));
        output.push_str(&format!("Count: {}\t", x.count));
        output.push_str("\n");
    }
    for x in res {
        let seqs = placecare::io::SearchedDescList::from(x.search_descs);
        output.push_str(&format!("{}\n", seqs));
    }
    output
}

fn print_query(res: Vec<Option<SeqDesc>>) -> String {
    let mut output = String::new();
    for (i, x) in res.iter().enumerate() {
        if let Some(x) = x {
            output.push_str(&format!("ID: {}\t", x.id));
            output.push_str(&format!("Accession: {}\t", x.ac));
            output.push_str(&format!("Description: {}\t", x.de));
            output.push_str("\n");
        } else {
            output.push_str(&format!("No result found<No.{}>\n", i));
        }
    }
    output
}

fn write_content(path: PathBuf, content: String) -> Result<(), std::io::Error> {
    if path.is_dir() {
        let file_path = path.join("output.txt");
        let mut file = fs::File::create(file_path)?;
        file.write_all(content.as_bytes())?;
    } else {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

#[derive(Parser)]
#[command(next_line_help = true)]
#[command(
    author = "bio-here",
    version = "1.0.0", 
    about = "Cis-regulatory element search tool using PLACE database", 
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "search", about = "Search for elements")]
    Search {
        // Input method: default is file
        // 0. file - to read from a file
        // 1. seq - to read from a sequence
        #[arg(
            short = 'i',
            long,
            conflicts_with("input_seq"),
            help = "Input file path",
            group = "inputs"
        )]
        input: Option<String>,

        #[arg(
            short = 's',
            long,
            conflicts_with("input"),
            help = "Input sequence",
            group = "inputs"
        )]
        input_seq: Option<String>,

        // Output method: default is print
        // 0. print - to print to stdout
        // 1. write - to write to a file uses the input path
        #[arg(short = 'p', long, help = "Output method: print")]
        print: bool,

        #[arg(short = 'w', long, help = "Output method: write")]
        write: bool,

        #[arg(
            short = 'o',
            long,
            required_if_eq("write", "true"),
            default_value = "output.txt",
            help = "Output file path"
        )]
        outfile: String,
    },

    #[command(name = "query", about = "Query the PLACE database")]
    Query {
        // Input method: default is file
        // 0. file - to read from a file
        // 1. seq - to read from a sequence
        #[arg(
            short = 'i',
            long,
            conflicts_with("input_seq"),
            help = "Input file path",
            group = "inputs"
        )]
        input: Option<String>,

        #[arg(
            short = 's',
            long,
            conflicts_with("input"),
            help = "Input string contains the query texts",
            group = "inputs"
        )]
        input_text: Option<String>,

        // Query method: default is id
        // 0. id - to query by id
        // 1. ac - to query by accession
        #[arg(short = 'q', long, conflicts_with("ac"), help = "Query method: id")]
        id: bool,

        #[arg(
            short = 'a',
            long,
            conflicts_with("id"),
            help = "Query method: accession"
        )]
        ac: bool,

        // Output method: default is print
        // 0. print - to print to stdout
        // 1. write - to write to a file uses the input path
        #[arg(short = 'p', long, help = "Output method: print")]
        print: bool,

        #[arg(short = 'w', long, help = "Output method: write")]
        write: bool,

        #[arg(
            short = 'o',
            long,
            required_if_eq("write", "true"),
            default_value = "output_query.txt",
            help = "Output file path"
        )]
        outfile: String,
    },
}
