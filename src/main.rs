/*
    Copyright 2020 The University of Edinburgh

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

    SPDX-License-Identifier: Apache-2.0
*/

use std::error::Error;
use structopt::StructOpt;

mod filter;
mod ompi;
mod smt;

#[derive(StructOpt)]
#[structopt(author)]
struct CliDemangle {
    #[structopt(
        name = "THREADS",
        required = true,
        help = "Intel-styled threads to demangle (no comma separated)."
    )]
    threads: Vec<u32>,
}

#[derive(StructOpt)]
#[structopt(author)]
struct CliGet {
    #[structopt(short = "t", long, help = "", possible_values = &filter::Thread::variants(), case_insensitive = true)]
    thread_rank: Option<filter::Thread>,
    #[structopt(short = "s", long, help = "", possible_values = &filter::Socket::variants(), case_insensitive = true)]
    socket: Option<filter::Socket>,
    #[structopt(short = "p", long, help = "", possible_values = &filter::CoreParity::variants(), case_insensitive = true)]
    parity: Option<filter::CoreParity>,
}

#[derive(StructOpt)]
enum CliCommand {
    /// Demangle a set of Intel-styled threads.
    Demangle(CliDemangle),
    /// Display a set of demangled threads that satisfy certain condition(s).
    Get(CliGet),
    /// Reverse demangle. From a set of demangled Intel-styled threads, return the threads in which the program will be finally executed.
    Mangle(CliDemangle),
}

#[derive(StructOpt)]
#[structopt(name = "dmglr", author, about)]
struct CliInit {
    #[structopt(long, default_value = "4", help = "SMT configuration (2 or 4).")]
    smt: smt::SMT,
    #[structopt(
        short = "m",
        long = "mpi-program",
        default_value = "<program>",
        help = "Append 'program' to MPI recommended command."
    )]
    program: String,
    #[structopt(subcommand)]
    cmd: CliCommand,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: CliInit = CliInit::from_args();

    match args.cmd {
        CliCommand::Demangle(dmg) => demangle_cmd(&args.smt, &args.program, dmg)?,
        CliCommand::Get(get) => get_cmd(&args.smt, &args.program, get)?,
        CliCommand::Mangle(mgl) => mangle_cmd(&args.smt, &args.program, mgl)?,
    }

    Ok(())
}

fn demangle_cmd(smt: &smt::SMT, program: &str, dmg: CliDemangle) -> Result<(), Box<dyn Error>> {
    let mut output = Vec::with_capacity(dmg.threads.len());

    for th in &dmg.threads {
        output.push(ompi::demangle(*th, smt)?);
    }

    output.sort();
    ompi::show_cmd(&output, program);

    Ok(())
}

fn mangle_cmd(smt: &smt::SMT, program: &str, dmg: CliDemangle) -> Result<(), Box<dyn Error>> {
    let mut output = Vec::with_capacity(dmg.threads.len());

    for th in &dmg.threads {
        output.push(ompi::mangle(*th, smt));
    }

    output.sort();
    ompi::show_cmd(&output, program);

    Ok(())
}

fn get_cmd(smt: &smt::SMT, program: &str, get: CliGet) -> Result<(), Box<dyn Error>> {
    let filter = filter::get_threads(smt, get.thread_rank, get.socket, get.parity)?;
    let mut output = Vec::with_capacity(filter.len());

    for th in filter {
        output.push(ompi::demangle(th, smt)?);
    }

    output.sort();

    ompi::show_cmd(&output, program);

    Ok(())
}
