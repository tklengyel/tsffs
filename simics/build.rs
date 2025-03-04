// Copyright (C) 2023 Intel Corporation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(feature = "link")]
    {
        use simics_api::sys::SIMICS_VERSION;
        use simics_link::link_simics_linux;

        #[cfg(target_family = "unix")]
        link_simics_linux(SIMICS_VERSION)?;

        #[cfg(not(target_family = "unix"))]
        compile_error!("Non-unix-like platforms are not yet supported");
    }

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
