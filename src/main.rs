extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate libzou;
extern crate num_cpus;

use clap::{App, Arg};
use libzou::cargo_helper::get_cargo_info;
use libzou::download::download_chunks;
use libzou::filesize::format_filesize;
use libzou::util::prompt_user;
use libzou::write::OutputFileWriter;
#[macro_use]
mod logs;
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;

fn main() {

    // Parse arguments

    let argparse = App::new("Zou")
        .about("Zou, a simple and fast download accelerator, written in Rust.")
        .version(crate_version!())
        .arg(Arg::with_name("file")
                 .long("file")
                 .short("f")
                 .takes_value(true)
                 .help("The local file to save the remote content file"))
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
        .arg(Arg::with_name("ssl_support")
                .long("ssl_support")
                .short("s")
                .help("Active the SSL Client to download files"))
        .arg(Arg::with_name("url")
            .index(1)
            //.multiple(true)
            .required(true))
        .get_matches();

    // Get informations from arguments

    // Get the URL as a Path structure
    let url = Path::new(argparse.value_of("url").unwrap());

    // Set the mirrors vectors
    let mut servers_urls: Vec<&str> = match url.parent() {
        Some(parent) => vec![parent.to_str().unwrap()],
        None => vec![],
    };
    // Set given mirrors
    if argparse.is_present("mirrors") {
        let mirrors_vec: Vec<&str> = argparse.values_of("mirrors").unwrap().collect();
        for mirror in mirrors_vec {
            servers_urls.push(mirror);
        }
    }

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
        info!(&format!("version: {}", crate_version!()));
        info!(&format!("file: {}", filename));
        info!(&format!("threads: {}", threads));
    }

    let local_path = Path::new(&filename);

    if local_path.exists() {
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

    let ssl_support = argparse.is_present("ssl_support");

    let cargo_info =
        get_cargo_info(filename, servers_urls, ssl_support).expect("fail to parse url");
    info!(&format!(
        "Remote content length: {}",
        format_filesize(cargo_info.content_length)
    ));

    let local_file = File::create(local_path).expect("[ERROR] Cannot create a file !");

    local_file.set_len(cargo_info.content_length).expect(
        "Cannot extend local file !",
    );
    let out_file = OutputFileWriter::new(local_file);

    // If the server does not accept PartialContent status, download the remote file
    // using only one thread
    if !cargo_info.accept_partialcontent {
        warning!(
            "The remote server does not accept PartialContent status! \
                             Downloading the remote file using one thread."
        );
        threads = 1;
    }

    if download_chunks(cargo_info, out_file, threads as u64, filename, ssl_support) {
        ok!(&format!(
            "Your download is available in {}",
            local_path.to_str().unwrap()
        ));
    } else {
        // If the file is not ok, delete it from the file system
        error!("Download failed! An error occured - erasing file... ");
        if remove_file(local_path).is_err() {
            error!("Cannot remove downloaded file!");
        }
    }

}
