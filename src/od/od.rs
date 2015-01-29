#![crate_name = "od"]
#![allow(unstable)]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Ben Hirsch <benhirsch24@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;
extern crate collections;

use collections::string::String;
use std::old_io::File;

#[derive(Show)]
enum Radix { Decimal, Hexadecimal, Octal, Binary }

pub fn uumain(args: Vec<String>) -> isize {
    let opts = [
        getopts::optopt("A", "address-radix",
                        "Select the base in which file offsets are printed.", "RADIX"),
        getopts::optopt("j", "skip-bytes",
                        "Skip bytes input bytes before formatting and writing.", "BYTES"),
        getopts::optopt("N", "read-bytes",
                        "limit dump to BYTES input bytes", "BYTES"),
        getopts::optopt("S", "strings",
                        ("output strings of at least BYTES graphic chars. 3 is assumed when \
                          BYTES is not specified."),
                        "BYTES"),
        getopts::optopt("t", "format", "select output format or formats", "TYPE"),
        getopts::optflag("v", "output-duplicates", "do not use * to mark line suppression"),
        getopts::optopt("w", "width",
                        ("output BYTES bytes per output line. 32 is implied when BYTES is not \
                          specified."),
                        "BYTES"),
        getopts::optflag("h", "help", "display this help and exit."),
        getopts::optflag("v", "version", "output version information and exit."),
    ];

    let matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(f) => panic!("Invalid options\n{}", f)
    };


    let mut rad = Radix::Octal;
    if matches.opt_present("A") {
        rad = parse_radix(matches.opt_str("A"));
    } else {
        println!("{}", getopts::usage("od", &opts));
    }

    let mut fname;
    match args.last() {
        Some(n) => fname = n,
        None    => { panic!("Need fname for now") ; }
    };

    main(rad, fname.clone());

    0
}

fn main(radix: Radix, fname: String) {
    let mut f = match File::open(&Path::new(fname)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e)
    };

    let mut addr = 0;
    let bytes = &mut [b'\x00'; 16];
    loop {
        match f.read(bytes) {
            Ok(n) => {
                print!("{:07o}", addr);
                match radix {
                    Radix::Decimal => {},
                    Radix::Octal => {
                        for b in range(0, n / std::u16::BYTES) {
                            let bs = &bytes[(2 * b) .. (2 * b + 2)];
                            let p: u16 = (bs[1] as u16) << 8 | bs[0] as u16;
                            print!(" {:06o}", p);
                        }
                        if n % std::u16::BYTES == 1 {
                            print!(" {:06o}", bytes[n - 1]);
                        }
                    }
                    _ => { }
                };
                print!("\n");
                addr += n;
            },
            Err(_) => { println!("{:07o}", addr); break; }
        };
    };
}

fn parse_radix(radix_str: Option<String>) -> Radix {
    let rad = match radix_str {
        Some(s) => {
            let st = s.into_bytes();
            if st.len() != 1 {
                panic!("Radix must be one of [d, o, b, x]\n");
            }

            let radix: char = *(st.get(0)
                                  .expect("byte string of length 1 lacks a 0th elem")) as char;
            if radix == 'd' {
                Radix::Decimal
            } else if radix == 'x' {
                Radix::Hexadecimal
            } else if radix == 'o' {
                Radix::Octal
            } else if radix == 'b' {
                Radix::Binary
            } else {
                panic!("Radix must be one of [d, o, b, x]\n");
            }
        },
        None => Radix::Octal
    };

    rad
}