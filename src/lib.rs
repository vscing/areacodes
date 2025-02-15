use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub use std::io::Result;

mod diff;
pub use diff::*;

pub fn for_each_line_in(path: impl AsRef<Path>, mut f: impl FnMut(&str)) -> Result<()> {
    let file = File::open(path)?;
    let mut br = BufReader::new(file);
    let mut buf = String::with_capacity(64);

    loop {
        match br.read_line(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                f(&buf);
                buf.clear();
            }
            Err(e) => return Err(e),
        }
    }
}

pub fn files(path: &str) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(path)
        .unwrap_or_else(|_| panic!("failed to read directory: {path}"))
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_file())
}

pub fn read_data(path: &impl AsRef<Path>, mut f: impl FnMut(u32, String)) -> Result<()> {
    let file_name = path.as_ref().file_name().unwrap().to_str().unwrap();
    for_each_line_in(path, |line| {
        let code = line
            .get(0..6)
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| panic!("invalid line in `{file_name}`: {line}"));
        assert_eq!(line.as_bytes()[6], b'\t');
        f(code, line[7..].into());
    })
}
