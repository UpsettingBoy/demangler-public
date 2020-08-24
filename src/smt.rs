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

use std::{error::Error, str::FromStr};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SMT {
    L2,
    L4,
}

impl SMT {
    pub fn get_jump(&self) -> u32 {
        match self {
            SMT::L2 => 32,
            SMT::L4 => 128,
        }
    }

    pub fn get_extra_threads(&self) -> u32 {
        match self {
            SMT::L2 => 64,
            SMT::L4 => 128,
        }
    }
}

impl From<&SMT> for u32 {
    fn from(smt: &SMT) -> Self {
        match smt {
            SMT::L2 => 2,
            SMT::L4 => 4,
        }
    }
}

impl FromStr for SMT {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numb = s.parse::<u8>()?;
        match numb {
            2 => Ok(SMT::L2),
            4 => Ok(SMT::L4),
            x @ _ => Err(From::from(format!("Invalid SMT level ({}).", x))),
        }
    }
}
