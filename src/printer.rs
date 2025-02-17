

use scanit::{BoxBytes, Receiver, ScanError};
use std::io::{stdout, Write, BufWriter, StdoutLock};
use memchr::memrchr;

const RESET: &[u8] = b"\x1b[0m";
const NEWLINE: &[u8] = b"\n";
const RESET_NL: &[u8] = b"\x1b[0m\n";
const COLOR_RS: &[u8] = b"\x1b[38;2;200;60;0m";
const COLOR_PY: &[u8] = b"\x1b[38;2;0;200;200m";
const COLOR_CPP: &[u8] = b"\x1b[38;2;0;100;200m";
const COLOR_H: &[u8] = b"\x1b[38;2;80;160;220m";
const COLOR_C: &[u8] = b"\x1b[38;2;255;255;224m";
const COLOR_LUA: &[u8] = b"\x1b[38;2;0;0;255m";
const COLOR_HTML: &[u8] = b"\x1b[38;2;255;105;180m";
const COLOR_CSS: &[u8] = b"\x1b[38;2;150;200;50m";
const COLOR_JS: &[u8] = b"\x1b[38;2;240;220;80m";
const COLOR_JSON: &[u8] = b"\x1b[38;2;160;140;200m";
const COLOR_TOML: &[u8] = b"\x1b[38;2;200;120;80m";
const COLOR_TXT: &[u8] = b"\x1b[38;2;128;128;128m";
const COLOR_MD: &[u8] = b"\x1b[38;2;100;180;100m";
const COLOR_INI: &[u8]   = b"\x1b[38;2;180;80;80m";
const COLOR_CFG: &[u8]   = b"\x1b[38;2;180;80;80m";
const COLOR_XML: &[u8]   = b"\x1b[38;2;130;90;200m";
const COLOR_YML: &[u8]   = b"\x1b[38;2;130;90;200m";
const COLOR_TS: &[u8]    = b"\x1b[38;2;90;150;250m";
const COLOR_SH: &[u8]    = b"\x1b[38;2;100;250;100m";
const COLOR_BAT: &[u8]   = b"\x1b[38;2;200;200;0m";
const COLOR_PS1: &[u8]   = b"\x1b[38;2;200;200;0m";
const COLOR_RB: &[u8]    = b"\x1b[38;2;200;0;200m";
const COLOR_PHP: &[u8]   = b"\x1b[38;2;80;80;200m";
const COLOR_PL: &[u8]    = b"\x1b[38;2;80;80;200m";
const COLOR_R: &[u8]     = b"\x1b[38;2;0;180;0m";
const COLOR_CS: &[u8]    = b"\x1b[38;2;50;50;50m";
const COLOR_JAVA: &[u8]  = b"\x1b[38;2;150;50;50m";
const COLOR_GO: &[u8]    = b"\x1b[38;2;0;150;150m";
const COLOR_SWIFT: &[u8] = b"\x1b[38;2;250;50;150m";
const COLOR_KT: &[u8]    = b"\x1b[38;2;50;150;250m";
const COLOR_SCSS: &[u8]  = b"\x1b[38;2;245;166;35m";
const COLOR_LESS: &[u8]  = b"\x1b[38;2;245;166;35m";
const COLOR_CSV: &[u8]   = b"\x1b[38;2;160;160;160m";
const COLOR_TSV: &[u8]   = b"\x1b[38;2;160;160;160m";
const COLOR_XLS: &[u8]   = b"\x1b[38;2;64;128;64m";
const COLOR_XLSX: &[u8]  = b"\x1b[38;2;64;128;64m";
const COLOR_SQL: &[u8]   = b"\x1b[38;2;100;100;100m";

#[inline]
fn extension_color(bytes: &[u8]) -> &'static [u8] {
    memrchr(b'.', bytes).map_or(RESET, |pos| {
        match &bytes[pos + 1..] {
            b"rs"    => COLOR_RS,
            b"py"    => COLOR_PY,
            b"cpp"   => COLOR_CPP,
            b"h"     => COLOR_H,
            b"c"     => COLOR_C,
            b"lua"   => COLOR_LUA,
            b"html"  => COLOR_HTML,
            b"css"   => COLOR_CSS,
            b"js"    => COLOR_JS,
            b"json"  => COLOR_JSON,
            b"toml"  => COLOR_TOML,
            b"txt"   => COLOR_TXT,
            b"md"    => COLOR_MD,
            b"ini"   => COLOR_INI,
            b"cfg"   => COLOR_CFG,
            b"xml"   => COLOR_XML,
            b"yml"   => COLOR_YML,
            b"ts"    => COLOR_TS,
            b"sh"    => COLOR_SH,
            b"bat"   => COLOR_BAT,
            b"ps1"   => COLOR_PS1,
            b"rb"    => COLOR_RB,
            b"php"   => COLOR_PHP,
            b"pl"    => COLOR_PL,
            b"r"     => COLOR_R,
            b"cs"    => COLOR_CS,
            b"java"  => COLOR_JAVA,
            b"go"    => COLOR_GO,
            b"swift" => COLOR_SWIFT,
            b"kt"    => COLOR_KT,
            b"scss"  => COLOR_SCSS,
            b"less"  => COLOR_LESS,
            b"csv"   => COLOR_CSV,
            b"tsv"   => COLOR_TSV,
            b"xls"   => COLOR_XLS,
            b"xlsx"  => COLOR_XLSX,
            b"sql"   => COLOR_SQL,
            _ => RESET,
        }
    })
}

#[inline]
fn write_plain(
    bytes: &[u8],
    writer: &mut BufWriter<StdoutLock>,
) -> Result<(), std::io::Error> {
    writer.write_all(bytes)?;
    writer.write_all(NEWLINE)
}



#[inline]
fn write_colored(
    bytes: &[u8],
    writer: &mut BufWriter<StdoutLock>,
    buffer: &mut Vec<u8>,
) -> Result<(), std::io::Error> {
    buffer.clear();
    buffer.extend_from_slice(extension_color(bytes));
    buffer.extend_from_slice(bytes);
    buffer.extend_from_slice(RESET_NL);
    writer.write_all(buffer)
}



#[cfg(unix)]
pub fn write_paths(
    paths: Receiver<BoxBytes>,
    limit: Option<usize>,
    colour: bool,
) -> Result<(), ScanError> {
    let mut writer = BufWriter::with_capacity(1024 * 64, stdout().lock());
    let mut line_buffer = Vec::with_capacity(512);
    let limit = limit.unwrap_or(usize::MAX);

    for path in paths.iter().take(limit) {
      
        
        if colour {
            write_colored(&path, &mut writer, &mut line_buffer)?;
        } else {
            write_plain(&path, &mut writer)?;
        }
        
       
    }

    writer.flush()?;
    Ok(())
}








#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(windows)]
pub fn write_paths(
    paths: Receiver<BoxBytes>,
    limit: Option<usize>,
) -> Result<(), ScanError> {
    let mut writer = BufWriter::with_capacity(1024 * 64, stdout().lock());
  
    let limit = limit.unwrap_or(usize::MAX);

    for path in paths.iter().take(limit) {
      
        
    
    
            write_plain(&path, &mut writer)?;
        
        
       
    }

    writer.flush()?;
    Ok(())
}
