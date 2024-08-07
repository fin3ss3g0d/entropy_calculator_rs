use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Calculate the Shannon entropy of a given string.
fn calculate_entropy(s: &str) -> f64 {
    let mut frequency: HashMap<char, usize> = HashMap::new();

    // Count the frequency of each character in the string
    for ch in s.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }

    // Calculate entropy
    let length = s.len() as f64;
    let mut entropy = 0.0;
    for &count in frequency.values() {
        let probability = count as f64 / length;
        entropy -= probability * probability.log2();
    }

    entropy
}

/// Calculate the Shannon entropy of a given string and check if it exceeds the threshold.
fn is_entropy_above_threshold(s: &str, threshold: f64) -> bool {
    let entropy = calculate_entropy(s);
    entropy > threshold
}

/// Function to write entropy levels of specific data sets to the output file.
fn test_entropy<W: Write>(s: &str, writer: &mut W) -> io::Result<()> {
    let entropy = calculate_entropy(s);
    writeln!(writer, "Entropy: {:.4} | String: {}", entropy, s)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <input_file_path> <output_file_path> [<entropy_threshold> | --test]",
            args[0]
        );
        std::process::exit(1);
    }
    
    let input_file_path = &args[1];
    let output_file_path = &args[2];
    
    // Determine if the test flag is used or if an entropy threshold is provided
    let test_flag = args.contains(&"--test".to_string());
    let entropy_threshold = if !test_flag && args.len() == 4 {
        Some(args[3].parse().map_err(|_| "Invalid entropy threshold")?)
    } else {
        None
    };

    if !test_flag && entropy_threshold.is_none() {
        eprintln!(
            "Error: Must provide either an entropy threshold or the --test flag"
        );
        std::process::exit(1);
    }

    // Open the input file for reading
    let input_file = File::open(input_file_path)?;
    let reader = BufReader::new(input_file);

    // Open the output file for writing
    let mut output_file = File::create(output_file_path)?;

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;
        
        if test_flag {
            // If test flag is present, write the entropy and line to the output file
            test_entropy(&line, &mut output_file)?;
        } else if let Some(threshold) = entropy_threshold {
            if is_entropy_above_threshold(&line, threshold) {
                writeln!(output_file, "{}", line)?;
            }
        }
    }

    println!("Entropy calculations completed and saved to {}", output_file_path);

    Ok(())
}
