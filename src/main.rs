extern crate clap; 
use clap::{App, Arg, ArgMatches};
use std::process::{Command, Stdio};

fn main() {

    let matches = arguments();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    // Input Command
    let mut input_args: Vec<&str> = input.split(" ").collect();
    let input_command = input_args.remove(0);

    let input_command = Command::new(&input_command[..])
                                .args(&input_args)
                                .output()
                                .expect("Error input command");

    let input = String::from_utf8_lossy(&input_command.stdout);


    // Output Command
    let mut output_args: Vec<&str> = output.split(" ").collect();
    let output_command = output_args.remove(0);

    let output_command = Command::new( output_command )
                                    .arg(&input[..])
                                    .args(output_args)
                                    .stdin(Stdio::piped())
                                    .output()
                                    .expect("Error Input Command");
    
    println!("{}", String::from_utf8_lossy(&output_command.stdout));
}

fn arguments() -> ArgMatches<'static> {
    let arguments =  App::new("mypipe")
        .version("1.0.0")
        .about("A Unix like command pipe in RUST")
        .author("Allan Maubert <a.maubert.jouvet@gmail.com>")
        .arg( Arg::with_name("input")
                .short("in")
                .long("input")
                .value_name("input")
                .help("input command")
                .takes_value(true)
                .required(true)
        )
        .arg( Arg::with_name("output")
                .short("out")
                .long("output")
                .value_name("output")
                .help("output command")
                .takes_value(true)
                .required(true)
        )   
        .get_matches();
    return  arguments;
}
