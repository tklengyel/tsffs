// Copyright (C) 2023 Intel Corporation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use raw_cstr::raw_cstr;
use simics_api_sys::SIM_run_command;

use crate::AttrValue;

pub fn run_command<S>(line: S) -> Result<AttrValue>
where
    S: AsRef<str>,
{
    Ok(unsafe { SIM_run_command(raw_cstr(line)?) })
}
