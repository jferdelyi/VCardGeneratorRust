use std::fmt::Write;
use std::fs::File;
use std::io::{Read, Write as ioWrite};
use std::path::{Path, PathBuf};
use std::{env, io};

/// Creates a simple VCard (version 2.1) for a contact.
///
/// This function generates a VCard as a `String` with the standard
/// fields for name, phone numbers, email, and a note. The VCard
/// can then be saved to a file or used as needed.
///
/// # Arguments
///
/// * `first_name` - Contact's first name.
/// * `last_name` - Contact's last name.
/// * `tel` - Contact's home phone number.
/// * `mobile` - Contact's mobile phone number.
/// * `email` - Contact's email address.
/// * `note` - Optional note about the contact.
///
/// # Returns
///
/// * `String` - A properly formatted VCard (version 2.1).
///
/// # Panics
///
/// This function **does not panic due to its own logic**. The only potential panics
/// come from the `writeln!` macro, which unwraps formatting errors. Since the
/// formatting strings are constant and safe, these panics should never occur.
///
/// # Examples
///
/// ```
/// let vcard = make_vcard(
///     "Alice",
///     "Smith",
///     "123-456-7890",
///     "098-765-4321",
///     "alice@example.com",
///     "Friend from school"
/// );
/// println!("{}", vcard);
/// ```
///
/// The output will be similar to:
///
/// ```text
/// BEGIN:VCARD
/// VERSION:2.1
/// N:Smith;Alice
/// FN:Alice Smith
/// EMAIL;PREF;INTERNET:alice@example.com
/// TEL;HOME;VOICE:123-456-7890
/// TEL;HOME;VOICE:098-765-4321
/// NOTE:Friend from school
/// REV:1
/// END:VCARD
/// ```
fn make_vcard(first_name: &str, last_name: &str, tel: &str, mobile: &str, email: &str, note: &str) -> String {
    let mut vcard = String::new();
    writeln!(vcard, "BEGIN:VCARD").unwrap();
    writeln!(vcard, "VERSION:2.1").unwrap();
    writeln!(vcard, "N:{last_name};{first_name}").unwrap();
    writeln!(vcard, "FN:{first_name} {last_name}").unwrap();
    writeln!(vcard, "EMAIL;PREF;INTERNET:{email}").unwrap();
    writeln!(vcard, "TEL;HOME;VOICE:{tel}").unwrap();
    writeln!(vcard, "TEL;HOME;VOICE:{mobile}").unwrap();
    writeln!(vcard, "NOTE:{note}").unwrap();
    writeln!(vcard, "REV:1").unwrap();
    writeln!(vcard, "END:VCARD").unwrap();
    vcard
}

/// Extracts VCard fields from a slice of strings and generates a VCard.
///
/// This function takes a slice of `String` representing contact data and maps
/// each element to a corresponding VCard field in the following order:
///
/// 1. First name
/// 2. Last name
/// 3. Home phone number
/// 4. Mobile phone number
/// 5. Email address
/// 6. Note
///
/// If any field is missing (slice shorter than 6 elements), it will be replaced
/// with an empty string. If the slice has more than 6 elements, extras are ignored.
///
/// # Arguments
///
/// * `vcard_data` - A slice of `String` containing the contact information.
///
/// # Returns
///
/// * `String` - A properly formatted VCard (version 2.1) as a `String`.
///
/// # Panics
///
/// This function **does not panic**. Missing fields are safely replaced by empty strings.
///
/// # Examples
///
/// ```
/// let data = vec![
///     "Alice".to_string(),
///     "Smith".to_string(),
///     "123-456-7890".to_string(),
///     "098-765-4321".to_string(),
///     "alice@example.com".to_string(),
///     "Friend from school".to_string()
/// ];
/// let vcard = extract_vcard_data(&data);
/// println!("{}", vcard);
/// ```
///
/// The output will be similar to:
///
/// ```text
/// BEGIN:VCARD
/// VERSION:2.1
/// N:Smith;Alice
/// FN:Alice Smith
/// EMAIL;PREF;INTERNET:alice@example.com
/// TEL;HOME;VOICE:123-456-7890
/// TEL;HOME;VOICE:098-765-4321
/// NOTE:Friend from school
/// REV:1
/// END:VCARD
/// ```
fn extract_vcard_data(vcard_data: &[String]) -> String {
    let first  = vcard_data.get(0).map(|s| s.as_str()).unwrap_or("");
    let last   = vcard_data.get(1).map(|s| s.as_str()).unwrap_or("");
    let tel    = vcard_data.get(2).map(|s| s.as_str()).unwrap_or("");
    let mobile = vcard_data.get(3).map(|s| s.as_str()).unwrap_or("");
    let email  = vcard_data.get(4).map(|s| s.as_str()).unwrap_or("");
    let note   = vcard_data.get(5).map(|s| s.as_str()).unwrap_or("");
    make_vcard(first, last, tel, mobile, email, note)
}

/// Writes the given data into a file.
///
/// This function will create the file if it does not exist, or
/// overwrite it if it already exists. The data is written as-is
/// from the provided string slice.
///
/// # Arguments
///
/// * `filename` - The path to the file. Can be a `&str`, `String`, `&Path`, or `PathBuf`.
/// * `data` - The string data to write into the file.
///
/// # Returns
///
/// * `Ok(())` - If the file was successfully written.
/// * `Err(io::Error)` - If the file could not be opened or written.
///
/// # Panics
///
/// This function **does not panic**; all errors are returned via `Result`.
///
/// # Examples
///
/// ```
/// write_file("output.txt", "Hello World!").unwrap();
/// ```
///
/// The file `output.txt` will contain:
///
/// ```text
/// Hello World!
/// ```
fn write_file<P: AsRef<Path>>(filename: P, data: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

/// Reads a CSV file and returns all lines as `Vec<Vec<String>>`.
///
/// Each line is split by the comma delimiter (`,`).
/// The first line (header) is skipped. This function reads the entire file into memory.
///
/// # Arguments
///
/// * `filename` - The path to the CSV file. Can be a `&str`, `String`, `&Path`, or `PathBuf`.
///
/// # Returns
///
/// * `Ok(Vec<Vec<String>>)` - Each line is represented as a `Vec<String>` containing its columns.
/// * `Err(io::Error)` - If the file cannot be opened or read.
///
/// # Panics
///
/// This function **does not panic**; all errors are returned via `Result`.
///
/// # Examples
///
/// ```
/// let records = read_csv_lines("data.csv").unwrap();
/// for line in records {
///     println!("{line:?}");
/// }
/// ```
///
/// If `data.csv` contains:
/// ```text
/// FIRSTNAME,LASTNAME,TEL,MOBILE,EMAIL,NOTE
/// John,Smith,,0612345678,john.smith@example.com,Friend from work
/// Jane,Doe,0987654321,,jane.doe@example.com,Colleague
/// Bob,Brown,0112233445,0611223344
/// ```
///
/// The returned value will be:
/// ```text
/// [
///     ["John", "Smith", "", "0612345678", "john.smith@example.com", "Friend from work"],
///     ["Jane", "Doe", "0987654321", "", "jane.doe@example.com", "Colleague"],
///     ["Bob", "Brown", "0112233445", "0611223344"],
/// ]
/// ```
fn read_csv_lines<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<String>>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let mut records = Vec::new();

    file.read_to_string(&mut contents)?;
    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let split_values: Vec<String> = line.split(',').map(|data| data.to_string()).collect();
        records.push(split_values);
    }

    Ok(records)
}


/// Builds an output file path based on an input file path and a desired extension.
///
/// This function takes an input file path (for example, a CSV file) and constructs
/// a new file path in the same directory with the same filename stem but a different
/// extension (for example, converting "data.csv" → "data.vcf").
///
/// # Arguments
///
/// * `input_path` - The path to the input file. Can be `&str`, `String`, `&Path`, or `PathBuf`.
/// * `output_extension` - The desired extension for the output file (without the dot).
///
/// # Returns
///
/// * `Some(PathBuf)` - A new `PathBuf` representing the output file path.
/// * `None` - If the input path has no parent directory or filename stem.
///
/// # Panics
///
/// This function **does not panic**; all errors are returned via `Result`.
///
/// # Examples
///
/// ```
/// let input_file = "contacts/my_contacts.csv";
/// let output_path = build_output_path(input_file, "vcf").unwrap();
/// assert_eq!(output_path.to_str().unwrap(), "contacts/my_contacts.vcf");
/// ```
///
/// ```
/// // Handles input paths in the current directory
/// let input_file = "my_contacts.csv";
/// if let Some(output_path) = build_output_path(input_file, "vcf") {
///     println!("Output file path: {:?}", output_path);
/// }
/// ```
fn build_output_path<P: AsRef<Path>>(input_path: P, output_extension: &str) -> io::Result<PathBuf> {
    let input_filename = input_path.as_ref();
    let input_parent = input_filename.parent().unwrap_or_else(|| { Path::new(".") }); // Not sure
    let csv_file_prefix = input_filename.file_stem().ok_or_else(||io::Error::new(io::ErrorKind::InvalidInput, "Input path has no file name"))?;

    let mut vcf_filename = PathBuf::from(input_parent);
    vcf_filename.push(csv_file_prefix);
    vcf_filename.set_extension(output_extension);

    Ok(vcf_filename)
}

/// This program reads a CSV file containing contact information and generates a VCard file for each contact.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run {} file.csv", args[0]);
        return Ok(());
    }
    let csv_filename = &args[1];
    let lines = read_csv_lines(csv_filename)?;
    let vcf_filename = build_output_path(csv_filename, "vcf")?;
    let all_vcard: Vec<String> = lines.iter().map(|element| extract_vcard_data(element)).collect();
    write_file(&vcf_filename, &all_vcard.join("\n"))
}
