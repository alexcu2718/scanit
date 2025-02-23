
use memchr::memrchr;
use scanit::{BoxBytes, Receiver, ScanError};

use std::io::{Write,BufWriter,stdout};


const NEWLINE:&[u8]=b"\n";
const RESET : &[u8] = b"\x1b[0m";
const COLOUR_RS : &[u8] = b"\x1b[38;2;200;60;0m";
const COLOUR_PY : &[u8] = b"\x1b[38;2;0;200;200m";
const COLOUR_CPP : &[u8] = b"\x1b[38;2;0;100;200m";
const COLOUR_H : &[u8] = b"\x1b[38;2;80;160;220m";
const COLOUR_C : &[u8] = b"\x1b[38;2;255;255;224m";
const COLOUR_LUA : &[u8] = b"\x1b[38;2;0;0;255m";
const COLOUR_HTML : &[u8] = b"\x1b[38;2;255;105;180m";
const COLOUR_CSS : &[u8] = b"\x1b[38;2;150;200;50m";
const COLOUR_JS : &[u8] = b"\x1b[38;2;240;220;80m";
const COLOUR_JSON : &[u8] = b"\x1b[38;2;160;140;200m";
const COLOUR_TOML : &[u8] = b"\x1b[38;2;200;120;80m";
const COLOUR_TXT : &[u8] = b"\x1b[38;2;128;128;128m";
const COLOUR_MD : &[u8] = b"\x1b[38;2;100;180;100m";
const COLOUR_INI : &[u8] = b"\x1b[38;2;180;80;80m";
const COLOUR_CFG : &[u8] = b"\x1b[38;2;180;80;80m";
const COLOUR_XML : &[u8] = b"\x1b[38;2;130;90;200m";
const COLOUR_YML : &[u8] = b"\x1b[38;2;130;90;200m";
const COLOUR_TS : &[u8] = b"\x1b[38;2;90;150;250m";
const COLOUR_SH : &[u8] = b"\x1b[38;2;100;250;100m";
const COLOUR_BAT : &[u8] = b"\x1b[38;2;200;200;0m";
const COLOUR_PS1 : &[u8] = b"\x1b[38;2;200;200;0m";
const COLOUR_RB : &[u8] = b"\x1b[38;2;200;0;200m";
const COLOUR_PHP : &[u8] = b"\x1b[38;2;80;80;200m";
const COLOUR_PL : &[u8] = b"\x1b[38;2;80;80;200m";
const COLOUR_R : &[u8] = b"\x1b[38;2;0;180;0m";
const COLOUR_CS : &[u8] = b"\x1b[38;2;50;50;50m";
const COLOUR_JAVA : &[u8] = b"\x1b[38;2;150;50;50m";
const COLOUR_GO : &[u8] = b"\x1b[38;2;0;150;150m";
const COLOUR_SWIFT : &[u8] = b"\x1b[38;2;250;50;150m";
const COLOUR_KT : &[u8] = b"\x1b[38;2;50;150;250m";
const COLOUR_SCSS : &[u8] = b"\x1b[38;2;245;166;35m";
const COLOUR_LESS : &[u8] = b"\x1b[38;2;245;166;35m";
const COLOUR_CSV : &[u8] = b"\x1b[38;2;160;160;160m";
const COLOUR_TSV : &[u8] = b"\x1b[38;2;160;160;160m";
const COLOUR_XLS : &[u8] = b"\x1b[38;2;64;128;64m";
const COLOUR_XLSX: &[u8] = b"\x1b[38;2;64;128;64m";
const COLOUR_SQL: &[u8] = b"\x1b[38;2;100;100;100m";






#[allow(clippy::inline_always)]
#[inline(always)]
fn extension_colour(bytes: &[u8]) ->&[u8] {
    memrchr(b'.', bytes).map_or(RESET, |pos| match &bytes[pos + 1..] {
        b"rs" => COLOUR_RS,
        b"py" => COLOUR_PY,
        b"cpp" => COLOUR_CPP,
        b"h" => COLOUR_H,
        b"c" => COLOUR_C,
        b"lua" => COLOUR_LUA,
        b"html" => COLOUR_HTML,
        b"css" => COLOUR_CSS,
        b"js" => COLOUR_JS,
        b"json" => COLOUR_JSON,
        b"toml" => COLOUR_TOML,
        b"txt" => COLOUR_TXT,
        b"md" => COLOUR_MD,
        b"ini"=>COLOUR_INI,
        b"cfg"=>COLOUR_CFG,
        b"xml" => COLOUR_XML,
        b"yml" => COLOUR_YML,
        b"ts" => COLOUR_TS,
        b"sh" => COLOUR_SH,
        b"bat" => COLOUR_BAT,
        b"ps1" => COLOUR_PS1,
        b"rb" => COLOUR_RB,
        b"php" => COLOUR_PHP,
        b"pl" => COLOUR_PL,
        b"r" => COLOUR_R,
        b"cs" => COLOUR_CS,
        b"java" => COLOUR_JAVA,
        b"go" => COLOUR_GO,
        b"swift" => COLOUR_SWIFT,
        b"kt" => COLOUR_KT,
        b"scss" => COLOUR_SCSS,
        b"less" => COLOUR_LESS,
        b"csv" => COLOUR_CSV,
        b"tsv" => COLOUR_TSV,
        b"xls" => COLOUR_XLS,
        b"xlsx" => COLOUR_XLSX,
        b"sql" => COLOUR_SQL,
        _ => RESET,
    })
}



#[allow(clippy::inline_always)]
#[inline(always)]
pub fn write_paths_plain(
    paths: &Receiver<BoxBytes>,
    limit: Option<usize>,
) -> Result<(), ScanError> {

    let mut buffer = BufWriter::new(stdout().lock());

    for path in paths.iter().take(limit.unwrap_or(usize::MAX)) {
        //unsafe {
            buffer.write_all(&path)?;
            buffer.write_all(NEWLINE)?;

    }

    buffer.flush()?;
    Ok(())
}

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn write_paths_coloured(
    paths: &Receiver<BoxBytes>,
    limit: Option<usize>,
) -> Result<(), ScanError> {

    let mut buffer = BufWriter::new(stdout().lock());

    for path in paths.iter().take(limit.unwrap_or(usize::MAX)) {


            buffer.write_all(extension_colour(&path))?;
            buffer.write_all(&path)?;
            buffer.write_all(NEWLINE)?;

    }


    buffer.flush()?;

    Ok(())
}



//commented out assembly implementation i did for lols


/* 
use memchr::memrchr;
use scanit::{BoxBytes, Receiver, ScanError};
#[cfg(target_arch = "x86_64")]
use std::arch::asm;

#[cfg(not(target_arch = "x86_64"))]
use std::io::Write;

const NEWLINE: &[u8] = b"\n";
const RESET: &[u8] = b"\x1b[0m";
const COLOUR_RS: &[u8] = b"\x1b[38;2;200;60;0m";
const COLOUR_PY: &[u8] = b"\x1b[38;2;0;200;200m";
const COLOUR_CPP: &[u8] = b"\x1b[38;2;0;100;200m";
const COLOUR_H: &[u8] = b"\x1b[38;2;80;160;220m";
const COLOUR_C: &[u8] = b"\x1b[38;2;255;255;224m";
const COLOUR_LUA: &[u8] = b"\x1b[38;2;0;0;255m";
const COLOUR_HTML: &[u8] = b"\x1b[38;2;255;105;180m";
const COLOUR_CSS: &[u8] = b"\x1b[38;2;150;200;50m";
const COLOUR_JS: &[u8] = b"\x1b[38;2;240;220;80m";
const COLOUR_JSON: &[u8] = b"\x1b[38;2;160;140;200m";
const COLOUR_TOML: &[u8] = b"\x1b[38;2;200;120;80m";
const COLOUR_TXT: &[u8] = b"\x1b[38;2;128;128;128m";
const COLOUR_MD: &[u8] = b"\x1b[38;2;100;180;100m";
const COLOUR_INI: &[u8] = b"\x1b[38;2;180;80;80m";
const COLOUR_CFG: &[u8] = b"\x1b[38;2;180;80;80m";
const COLOUR_XML: &[u8] = b"\x1b[38;2;130;90;200m";
const COLOUR_YML: &[u8] = b"\x1b[38;2;130;90;200m";
const COLOUR_TS: &[u8] = b"\x1b[38;2;90;150;250m";
const COLOUR_SH: &[u8] = b"\x1b[38;2;100;250;100m";
const COLOUR_BAT: &[u8] = b"\x1b[38;2;200;200;0m";
const COLOUR_PS1: &[u8] = b"\x1b[38;2;200;200;0m";
const COLOUR_RB: &[u8] = b"\x1b[38;2;200;0;200m";
const COLOUR_PHP: &[u8] = b"\x1b[38;2;80;80;200m";
const COLOUR_PL: &[u8] = b"\x1b[38;2;80;80;200m";
const COLOUR_R: &[u8] = b"\x1b[38;2;0;180;0m";
const COLOUR_CS: &[u8] = b"\x1b[38;2;50;50;50m";
const COLOUR_JAVA: &[u8] = b"\x1b[38;2;150;50;50m";
const COLOUR_GO: &[u8] = b"\x1b[38;2;0;150;150m";
const COLOUR_SWIFT: &[u8] = b"\x1b[38;2;250;50;150m";
const COLOUR_KT: &[u8] = b"\x1b[38;2;50;150;250m";
const COLOUR_SCSS: &[u8] = b"\x1b[38;2;245;166;35m";
const COLOUR_LESS: &[u8] = b"\x1b[38;2;245;166;35m";
const COLOUR_CSV: &[u8] = b"\x1b[38;2;160;160;160m";
const COLOUR_TSV: &[u8] = b"\x1b[38;2;160;160;160m";
const COLOUR_XLS: &[u8] = b"\x1b[38;2;64;128;64m";
const COLOUR_XLSX: &[u8] = b"\x1b[38;2;64;128;64m";
const COLOUR_SQL: &[u8] = b"\x1b[38;2;100;100;100m";

#[allow(clippy::inline_always)]
#[inline(always)]
fn extension_colour(bytes: &[u8]) -> &[u8] {
    memrchr(b'.', bytes).map_or(RESET, |pos| match &bytes[pos + 1..] {
        b"rs" => COLOUR_RS,
        b"py" => COLOUR_PY,
        b"cpp" => COLOUR_CPP,
        b"h" => COLOUR_H,
        b"c" => COLOUR_C,
        b"lua" => COLOUR_LUA,
        b"html" => COLOUR_HTML,
        b"css" => COLOUR_CSS,
        b"js" => COLOUR_JS,
        b"json" => COLOUR_JSON,
        b"toml" => COLOUR_TOML,
        b"txt" => COLOUR_TXT,
        b"md" => COLOUR_MD,
        b"ini" => COLOUR_INI,
        b"cfg" => COLOUR_CFG,
        b"xml" => COLOUR_XML,
        b"yml" => COLOUR_YML,
        b"ts" => COLOUR_TS,
        b"sh" => COLOUR_SH,
        b"bat" => COLOUR_BAT,
        b"ps1" => COLOUR_PS1,
        b"rb" => COLOUR_RB,
        b"php" => COLOUR_PHP,
        b"pl" => COLOUR_PL,
        b"r" => COLOUR_R,
        b"cs" => COLOUR_CS,
        b"java" => COLOUR_JAVA,
        b"go" => COLOUR_GO,
        b"swift" => COLOUR_SWIFT,
        b"kt" => COLOUR_KT,
        b"scss" => COLOUR_SCSS,
        b"less" => COLOUR_LESS,
        b"csv" => COLOUR_CSV,
        b"tsv" => COLOUR_TSV,
        b"xls" => COLOUR_XLS,
        b"xlsx" => COLOUR_XLSX,
        b"sql" => COLOUR_SQL,
        _ => RESET,
    })
}

const BUFFER_SIZE: usize = 8192/2; // 8KB buffer

struct WriteBuffer {
    buffer: Vec<u8>,
}

impl WriteBuffer {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(BUFFER_SIZE),
        }
    }
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn write(&mut self, data: &[u8]) -> Result<(), ScanError> {
        if self.buffer.len() + data.len() > BUFFER_SIZE {
            self.flush()?;
        }
        self.buffer.extend_from_slice(data);
        if self.buffer.len() >= BUFFER_SIZE {
            self.flush()?;
        }
        Ok(())
    }
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn flush(&mut self) -> Result<(), ScanError> {
        if !self.buffer.is_empty() {
            let ret: i64;
            asm!(
                "syscall",
                in("rax") 1,
                in("rdi") 1,
                in("rsi") self.buffer.as_ptr(),
                in("rdx") self.buffer.len(),
                lateout("rax") ret,
                out("rcx") _,
                out("r11") _,
                options(nostack)   // hint that we don't touch the stack
            );
            self.buffer.clear();
            if ret < 0 {
                return Err(ScanError::Io(std::io::Error::last_os_error()));
            }
        }
        Ok(())
    }
}

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn write_paths_plain(
    paths: &Receiver<BoxBytes>,
    limit: Option<usize>,
) -> Result<(), ScanError> {
    #[cfg(target_arch = "x86_64")]
    let mut buffer = WriteBuffer::new();
    #[cfg(not(target_arch = "x86_64"))]
    let mut buffer = io::BufWriter::new(io::stdout().lock());

    for path in paths.iter().take(limit.unwrap_or(usize::MAX)) {
        unsafe {
            buffer.write(&path)?;
            buffer.write(NEWLINE)?;
        }
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        buffer.flush()?;
    };
    #[cfg(not(target_arch = "x86_64"))]
    buffer.flush()?;
    Ok(())
}

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn write_paths_coloured(
    paths: &Receiver<BoxBytes>,
    limit: Option<usize>,
) -> Result<(), ScanError> {
    #[cfg(target_arch = "x86_64")]
    let mut buffer = WriteBuffer::new();
    #[cfg(not(target_arch = "x86_64"))]
    let mut buffer = io::BufWriter::new(io::stdout().lock());

    for path in paths.iter().take(limit.unwrap_or(usize::MAX)) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            buffer.write(extension_colour(&path))?;
            buffer.write(&path)?;
            buffer.write(NEWLINE)?;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            buffer.write(extension_colour(&path))?;
            buffer.write(&path)?;
            buffer.write(NEWLINE)?;
        }
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        buffer.flush()?;
    };
    #[cfg(not(target_arch = "x86_64"))]
    buffer.flush()?;

    Ok(())
}
*/