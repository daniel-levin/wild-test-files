#![allow(dead_code)]

use rayon::prelude::*;
use serde::Deserialize;
use sha2::Digest;
use std::path::Path;
use std::path::PathBuf;

/// Include more licenses as and when they are needed.
static ALLOWED_LICENSES: [&str; 2] = ["CDDL-1.0", "LGPL-2.1-or-later"];

#[derive(Debug, Deserialize)]
struct Artifact {
    location: PathBuf,
    #[serde(deserialize_with = "hex::serde::deserialize")]
    sha2: [u8; 32],

    license: String,
}

impl Artifact {
    fn validate(&self) -> anyhow::Result<()> {
        let contents = std::fs::read(Path::new("../").join(&self.location))?;
        let actual_sha2: [u8; 32] = sha2::Sha256::digest(&contents).into();

        if actual_sha2 != self.sha2 {
            let actual_s = hex::encode(&actual_sha2);
            let expected_s = hex::encode(&self.sha2);
            anyhow::bail!(
                "artifact {:#?} has sha256 {} but {} was expected",
                &self.location,
                actual_s,
                expected_s
            );
        }

        let Some(license) = spdx::license_id(&self.license) else {
            anyhow::bail!("Unknown license {}", self.license);
        };

        if !ALLOWED_LICENSES.contains(&license.name) {
            anyhow::bail!(
                "License {} not allowed. Allowed licenses: {:#?}",
                license.name,
                &ALLOWED_LICENSES
            );
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    artifact: Vec<Artifact>,
}

impl Manifest {
    fn validate(&self) -> anyhow::Result<()> {
        let _ = self
            .artifact
            .par_iter()
            .map(Artifact::validate)
            .collect::<anyhow::Result<Vec<()>>>()?;

        Ok(())
    }
}

#[test]
fn selftest() -> anyhow::Result<()> {
    let manifest: Manifest = toml::from_slice(include_bytes!("../../Manifest.toml"))?;

    manifest.validate()
}
