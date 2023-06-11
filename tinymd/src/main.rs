use std::fs::File;
use std::io::{Write,BufRead, BufReader};
use std::path::Path;

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[Info] Trying to parse {}...", _filename);

    // Create a path variable from the filename
    let input_filename: &Path = Path::new(_filename);

    // Try to open the file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");
    let mut ptag: bool = false;
    let mut htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }

                if htag {
                    output_line.push_str("</h1>\n");
                }

                htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                if !ptag {
                    // If ptag is false,
                    ptag = true; // set it to true, then
                    output_line.push_str("<p>"); // push a <p> to the output line.
                }

                output_line.push_str(&line_contents); // Push the whole line to the output line.
            }
        }
				if ptag {
					ptag = false;
					output_line.push_str("</p>\n");
				}
				
				if htag {
					htag = false;
					output_line.push_str("</h1>\n");      
				}

				if output_line != "<p></p>\n" {
					tokens.push(output_line);
				}
    }

		
		let mut output_filename= String::from(&_filename[.._filename.len()-3]);
		output_filename.push_str(".html");
		
		let mut outfile = File::create(output_filename).expect("[Error] could not create output file");
		
		for token in &tokens{
			outfile.write_all(token.as_bytes()).expect("[Error] Could not write to output file!");
		}

		println!("[ INFO ] Parsing complete!");


}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[Error] Invalid Invocation (you done goofed!)");
            usage();
        }
    }
    // usage();
}
