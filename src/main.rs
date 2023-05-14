use clap::{arg, value_parser, Command};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Error, Debug)]
enum AppError {
    #[error("parsing url faild {0}")]
    UrlParseError(#[from] ParseError),
    #[error("the parameter {0} not found")]
    ParameterNotFound(String),
    #[error("File Error")]
    FileError(#[from] io::Error),
}

fn replace_url_param(
    url_str: &str,
    param_name: Option<&String>,
    new_value: &String,
) -> Result<String, AppError> {
    let mut parsed_url = Url::parse(url_str)?;
    let mut hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    if let Some(name) = param_name {
        match hash_query.get(name) {
            Some(_) => {
                hash_query.insert(name.to_string(), new_value.to_string());
            }
            None => return Err(AppError::ParameterNotFound(name.to_string())),
        }
    } else {
        for (_, value) in hash_query.iter_mut() {
            *value = new_value.to_owned();
        }
    }

    parsed_url
        .query_pairs_mut()
        .clear()
        .extend_pairs(hash_query);
    Ok(parsed_url.to_string())
}

fn main() -> Result<(), AppError> {
    let matches = Command::new("Parameter Replacer")
        .about("Parameter
         Replace Values")
        .version("1.0")
        .author("Alirizap")
        .arg(
            arg!(-f --file <FILE> "File contain urls")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-n --new_value <NEW_VALUE> "New value to replace with default parameter values")
                .default_value("1337"),
        )
        .arg(arg!(-p --parameter <PARAMETER> "Parameter to replace display urls with this parameter, default all parameters"))
        .arg(arg!(-r --report_errors "Report errors and exit the program default skip url parsing errors"))
        .get_matches();

    let file_path = matches.get_one::<PathBuf>("file").unwrap();
    let new_value = matches.get_one::<String>("new_value").unwrap();
    let parameter = matches.get_one::<String>("parameter");
    let report = matches.get_flag("report_errors");

    let reader = BufReader::new(File::open(file_path)?);

    for line in reader.lines() {
        let line = line?;
        match replace_url_param(&line, parameter, new_value) {
            Ok(url) => println!("{url}"),
            Err(e) if report => return Err(e),
            Err(_) => (),
        }
    }

    Ok(())
}
