extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate libzou;
extern crate num_cpus;

use clap::{App, Arg};
use libzou::cargo_helper::get_remote_server_informations;
use libzou::download::download_chunks;
use libzou::filesize::StringFileSize;
use libzou::protocol::{get_protocol, Protocol};
use libzou::util::prompt_user;
use libzou::write::OutputFileWriter;
#[macro_use]
mod logs;
use std::error::Error;
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;

fn main() {

    // Parse arguments

    let argparse = App::new("Zou")
        .about("Zou, a simple and fast download accelerator, written in Rust.")
        .version(crate_version!())
        .arg(Arg::with_name("threads")
                 .long("threads")
                 .short("t")
                 .takes_value(true)
                 .help("Threads which can use to download"))
        .arg(Arg::with_name("debug")
                 .long("debug")
                 .short("d")
                 .help("Active the debug mode"))
        .arg(Arg::with_name("force")
                 .long("force")
                 .help("Assume Yes to all queries and do not prompt"))
        .arg(Arg::with_name("mirrors")
                 .long("mirrors")
                 .short("m")
                 .multiple(true)
                 .takes_value(true)
                 .help("Download using a list of mirrors - the list of mirrors is used WITH the original URL"))
        .arg(Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .help("Specify the local output"))
        .arg(Arg::with_name("ssl_support")
                .long("ssl_support")
                .short("s")
                .help("Switch to an SSL client"))
        .arg(Arg::with_name("url")
            .index(1)
            //.multiple(true)
            .required(true))
        .get_matches();

    // Get informations from arguments

    // Get the URL as a Path structure
    let url = Path::new(argparse.value_of("url").unwrap());
    let url_str = url.to_str().unwrap();

    // Get the path filename
    let filename = url.file_name().unwrap().to_str().unwrap();

    // Check if multi-threaded download is possible
    let mut threads: usize = value_t!(argparse, "threads", usize)
        .and_then(|v| if v != 0 {
            Ok(v)
        } else {
            Err(clap::Error::with_description(
                "Cannot download a file using 0 thread",
                clap::ErrorKind::InvalidValue,
            ))
        })
        .unwrap_or(num_cpus::get_physical());

    if argparse.is_present("debug") {
        info!(&format!("zou V{}", crate_version!()));
        info!(&format!(
            "downloading {}, using {} threads",
            filename,
            threads
        ));
    }

    let local_path = Path::new(argparse.value_of("output").unwrap_or(&filename));

    // Check for an existing file if the 'resume' option has been set
    if argparse.is_present("resume") {
        info!("Asking to resume a download...");
        if ! local_path.exists() {
            warning!("The download does not exists! It will start from 0.");
            let user_input = prompt_user(
                "Do you want to start the download from 0? [y/N]"
            );
            if (user_input == "n" || user_input == "N") {
                exit(0);
            }
        } else {
            
        }
    }

    // Override existing file if the 'resume' option has not been set
    if !argparse.is_present("resume") && local_path.exists() {
        if local_path.is_dir() {
            epanic!(
                "The local path to store the remote content is already exists, \
                        and is a directory!"
            );
        }
        if !argparse.is_present("force") {
            let user_input = prompt_user(
                "The path to store the file already exists! \
                                          Do you want to override it? [y/N]",
            );
            if !(user_input == "y" || user_input == "Y") {
                exit(0);
            }
        } else {
            warning!(
                "The path to store the file already exists! \
                                 It is going to be overriden."
            );
        }
    }

    // Get automaticaly the protocol from the given URL
    let ssl_support = match get_protocol(url.to_str().unwrap()) {
        Some(protocol) => {
            match protocol {
                // If the protocol is HTTP, return the user decision for the HTTPS client
                Protocol::HTTP => argparse.is_present("ssl_support"),
                // Force to use HTTPS client
                Protocol::HTTPS => true,
            }
        }
        None => epanic!("Unknown protocol!"),
    };

    // Get remote server informations in order to perform the best download strategy as possible
    let remote_server_informations = match get_remote_server_informations(url_str, ssl_support) {
        Ok(mut informations) => {
            // Check if the user asked for monothreading download
            informations.accept_partialcontent = !(threads == 1);
            // Return the data structure
            informations
        }
        Err(err) => {
            error!(&format!(
                "Getting remote server informations: {}",
                err.description()
            ));
            exit(1);
        }
    };

    info!(&format!(
        "Remote content length: {}",
        StringFileSize::from(
            remote_server_informations.file.content_length,
        )
    ));

    let local_file = File::create(local_path).expect("[ERROR] Cannot create a file !");

    local_file
        .set_len(remote_server_informations.file.content_length)
        .expect("Cannot extend local file !");
    let out_file = OutputFileWriter::new(local_file);

    // If the server does not accept PartialContent status, download the remote file
    // using only one thread
    if !remote_server_informations.accept_partialcontent {
        warning!(
            "The remote server does not accept PartialContent status! \
                             Downloading the remote file using one thread."
        );
        threads = 1;
    }

    if download_chunks(remote_server_informations, out_file, threads as u64, ssl_support)
    {
        ok!(&format!(
            "Your download is available in {}",
            local_path.to_str().unwrap()
        ));
    } else {
        // If the file is not ok, delete it from the file system
        error!("Download failed! An error occured - erasing file... ");
        if remove_file(local_path).is_err() {
            error!("Cannot delete downloaded file!");
        }
    }

}
