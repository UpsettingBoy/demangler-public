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

use crate::{ompi::CORE_NUMBER, smt};
use clap::arg_enum;
use std::error::Error;

arg_enum! {
    #[derive(Debug)]
    pub enum Thread {
        First,
        Second,
        Third,
        Fourth,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum Socket {
        N0,
        N1,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum CoreParity {
        Odd,
        Even,
    }
}

impl Thread {
    pub fn coherent(&self, smt: &smt::SMT) -> bool {
        match self {
            Thread::First if *smt == smt::SMT::L2 || *smt == smt::SMT::L4 => true,
            Thread::Second if *smt == smt::SMT::L2 || *smt == smt::SMT::L4 => true,
            Thread::Third if *smt == smt::SMT::L4 => true,
            Thread::Fourth if *smt == smt::SMT::L4 => true,
            _ => false,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Thread::First => 0,
            Thread::Second => 1,
            Thread::Third => 2,
            Thread::Fourth => 3,
        }
    }
}

pub fn get_threads(
    smt: &smt::SMT,
    th: Option<Thread>,
    sck: Option<Socket>,
    p: Option<CoreParity>,
) -> Result<Vec<u32>, Box<dyn Error>> {
    let smt_val = u32::from(smt);
    let mut table = generate_table(smt);

    if let Some(sck_filter) = sck {
        let tmp = match sck_filter {
            Socket::N0 => &table[..smt.get_extra_threads() as usize],
            Socket::N1 => &table[smt.get_extra_threads() as usize..],
        };

        table = tmp.to_owned();
    }

    if let Some(th_filter) = th {
        if !th_filter.coherent(smt) {
            return Err(From::from(format!(
                "-t {:?} filter option cannot be used with SMT{}",
                th_filter, smt_val
            )));
        }

        let mut tmp = Vec::new();
        for chunk in table.chunks(smt_val as usize) {
            if let Some(ind) = chunk.get(th_filter.index()) {
                tmp.push(*ind);
            } else {
                return Err(From::from(format!(
                    "Cannot get index {} while filtering!",
                    th_filter.index()
                )));
            }
        }

        table = tmp;
    }

    if let Some(p_filter) = p {
        let tmp = table
            .into_iter()
            .filter(|val| match p_filter {
                CoreParity::Odd => *val % 2 == 1,
                CoreParity::Even => *val % 2 == 0,
            })
            .collect();

        table = tmp;
    }

    Ok(table)
}

fn generate_table(smt: &smt::SMT) -> Vec<u32> {
    let smt_val = u32::from(smt);
    let extra_val = smt.get_extra_threads();
    let mut table = vec![0; (smt_val * 2 * CORE_NUMBER) as usize];

    let mut start_val = 0_u32;
    for (i, pos) in table.chunks_mut(smt_val as usize).enumerate() {
        let mut curr_val = start_val;
        for (j, el) in pos.iter_mut().enumerate() {
            if (i as u32 * smt_val) + j as u32 == extra_val {
                start_val = CORE_NUMBER * smt_val;
                curr_val = start_val;
            }

            *el = curr_val;
            curr_val += CORE_NUMBER;
        }

        start_val += 1;
    }

    table
}
