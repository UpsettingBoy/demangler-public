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

use crate::smt;
use std::{error::Error, fmt::Display};

pub const CORE_NUMBER: u32 = 32;

#[allow(dead_code)]
pub fn mangle(input: u32, smt: &smt::SMT) -> u32 {
    let smt_val = u32::from(smt);
    let row_index = input / smt_val;
    let col_index = input % smt_val;

    let mut start = match col_index {
        0 => 0,
        1 => 32,
        2 => 64,
        3 => 96,
        _ => panic!("Mangle panic!"),
    };

    let jump = smt.get_jump();
    if input >= CORE_NUMBER * smt_val {
        start += jump;
    }

    match smt {
        smt::SMT::L2 => start + row_index,
        smt::SMT::L4 => start + (row_index % 32),
    }
}

pub fn demangle(input: u32, smt: &smt::SMT) -> Result<u32, Box<dyn Error>> {
    let smt_val = u32::from(smt);

    if input > (smt_val * 64_u32) - 1_u32 {
        return Err(From::from(format!(
            "{} cannot be parsed! Greater than nº of hw threads.",
            input
        )));
    }

    let jump = smt.get_extra_threads();

    let mut flag = false;
    let thread_value = if input >= jump {
        flag = true;
        input - jump
    } else {
        input
    };

    let col_index = thread_value / CORE_NUMBER;
    let row_index = thread_value % CORE_NUMBER;

    let output = smt_val * row_index + col_index + if flag { jump } else { 0 };

    Ok(output)
}

pub fn show_cmd<T: Display>(input: &[T], prog: &str) {
    println!(
        "mpirun -n {} --use-hwthread-cpus -bind-to cpu-list:ordered -cpu-set {} {}",
        input.len(),
        format_vector(&input),
        prog
    );
}

fn format_vector<T: Display>(input: &[T]) -> String {
    let mut output = String::with_capacity(2 * input.len() - 1);

    for (i, n) in input.iter().enumerate() {
        let parsed = n.to_string();

        output.push_str(parsed.as_str());

        if i < input.len() - 1 {
            output.push(',');
        }
    }

    output
}
