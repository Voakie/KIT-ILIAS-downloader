// SPDX-License-Identifier: GPL-2.0-or-later

use anyhow::Context;
use tokio::fs::File as AsyncFile;
use tokio::io::{AsyncRead, BufWriter};

use std::path::Path;

use crate::Result;

/// Write all data to the specified path. Will overwrite previous file data.
pub async fn write_file_data<R: ?Sized>(path: impl AsRef<Path>, data: &mut R) -> Result<()> 
where R: AsyncRead + Unpin {
	let file = AsyncFile::create(path.as_ref()).await.context("failed to create file")?;
	let mut file = BufWriter::new(file);
	tokio::io::copy(data, &mut file).await.context("failed to write to file")?;
	Ok(())
}

/// Create a directory. Does not error if the directory already exists.
pub async fn create_dir(path: &Path) -> Result<()> {
	if let Err(e) = tokio::fs::create_dir(&path).await {
		if e.kind() != tokio::io::ErrorKind::AlreadyExists {
			return Err(e.into());
		}
	}
	Ok(())
}

const INVALID: &[char] = &['/', '\\', ':', '<', '>', '"', '|', '?', '*', '\n', '\t'];

pub fn file_escape(s: &str) -> String {
	s.replace(INVALID, "-")
}
