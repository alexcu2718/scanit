
const BUFFER_SIZE: usize = 1024 * 1024 / 8;
const NEWLINE: &[u8; 1] = b"\n";

use scanit::{BoxBytes, IntoIter, ScanError};
use std::io::{stdout, Write, BufWriter};
use nu_ansi_term::Color::Rgb;
use nu_ansi_term::Color;
use std::io::StdoutLock;

/*
use std::collections::HashMap;
use std::sync::OnceLock;
#[cfg(unix)]
static COLOR_MAP: OnceLock<HashMap<&'static [u8], Color>> = OnceLock::new();

#[cfg(unix)]
fn get_color_map() -> &'static HashMap<&'static [u8], Color> {
    COLOR_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Programming languages
        m.insert(&b".rs"[..], Rgb(200, 60, 0));
        m.insert(&b".py"[..], Rgb(0, 200, 200));
        m.insert(&b".cpp"[..], Rgb(0, 100, 200));
        m.insert(&b".h"[..], Rgb(80, 160, 220));
        m.insert(&b".c"[..], Rgb(255, 255, 224));
        m.insert(&b".lua"[..], Rgb(0, 0, 255));
        // Web formats
        m.insert(&b".html"[..], Rgb(255, 105, 180));
        m.insert(&b".css"[..], Rgb(150, 200, 50));
        m.insert(&b".js"[..], Rgb(240, 220, 80));
        // Data formats
        m.insert(&b".json"[..], Rgb(160, 140, 200));
        m.insert(&b".toml"[..], Rgb(200, 120, 80));
        // Text formats
        m.insert(&b".txt"[..], Rgb(128, 128, 128));
        m.insert(&b".md"[..], Rgb(100, 180, 100));
        m
    })
}



#[cfg(unix)]
#[inline]
fn detect_color(bytes: &[u8]) -> Color {
    bytes.iter().rposition(|&b| b == b'.')
        .and_then(|pos| get_color_map().get(&bytes[pos..]))
        .copied()
        .unwrap_or(Color::Default)
}




*/




#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(unix)]
fn write_color(bytes: &[u8], bufwrite: &mut BufWriter<StdoutLock<'static>>) -> Result<(), std::io::Error> {
     bytes.iter()
        .rposition(|&b| b == b'.')
        .map_or(Color::Default, |pos| {
            match &bytes[pos..] {
                b".rs" => Rgb(200, 60, 0),
                b".py" => Rgb(0, 200, 200),
                b".cpp" => Rgb(0, 100, 200),
                b".h" => Rgb(80, 160, 220),
                b".c" => Rgb(255, 255, 224),
                b".lua" => Rgb(0, 0, 255),
                b".html" => Rgb(255, 105, 180),
                b".css" => Rgb(150, 200, 50),
                b".js" => Rgb(240, 220, 80),
                b".json" => Rgb(160, 140, 200),
                b".toml" => Rgb(200, 120, 80),
                b".txt" => Rgb(128, 128, 128),
                b".md" => Rgb(100, 180, 100),
                _ => Color::Default,
            }
        }).paint(bytes).write_to(bufwrite)
   
}


#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(unix)]
pub fn write_paths(
    paths: IntoIter<BoxBytes>,
    limit: Option<usize>,
    colour: bool,
) -> Result<(), ScanError> {
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout().lock());

    if colour {
        for path in paths.take(limit.unwrap_or(usize::MAX)) {
            write_color(&path,&mut writer)?;
        
            writer.write_all(NEWLINE)?;
        }
    } else {
        for path in paths.take(limit.unwrap_or(usize::MAX)) {
            writer.write_all(&path)?;
            writer.write_all(NEWLINE)?;
        }
    }

    writer.flush()?;
    Ok(())
}



/*

#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(unix)]
pub fn write_paths(
    paths: IntoIter<BoxBytes>,
    limit: Option<usize>,
    colour: bool,
) -> Result<(), ScanError> {
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout().lock());

    if colour {
        for path in paths.take(limit.unwrap_or(usize::MAX)) {
            let color = detect_color(&path);
            color.paint(&*path).write_to(&mut writer)?;
            writer.write_all(NEWLINE)?;
        }
    } else {
        for path in paths.take(limit.unwrap_or(usize::MAX)) {
            writer.write_all(&path)?;
            writer.write_all(NEWLINE)?;
        }
    }

    writer.flush()?;
    Ok(())
}

*/

















#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(windows)]
pub fn write_paths(paths: IntoIter<BoxBytes>, limit: Option<usize>) -> Result<(), ScanError> {
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout().lock());
    for path in paths.take(limit.unwrap_or(usize::MAX)) {
        writer.write_all(&path)?;
        writer.write_all(NEWLINE)?;
    }
    writer.flush()?;
    Ok(())
}
