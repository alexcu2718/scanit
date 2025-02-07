






use scanit::{IntoIter, ArcStr,io::{Result,stdout, Write, BufWriter},FLUSH_THRESHOLD,BUFFER_SIZE,NEWLINE};


#[allow(clippy::inline_always)]
#[inline(always)]
pub fn write_paths(paths: IntoIter<ArcStr>, limit: Option<usize>) -> Result<()> {
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, stdout());
    if limit.is_some(){
    for path in paths.take(limit.unwrap()) {
        writer.write_all(path.as_bytes())?;
        writer.write_all(NEWLINE)?;
        if writer.buffer().len() >= FLUSH_THRESHOLD {writer.flush()?;}
    }
    writer.flush()?;
    //early return here
    return Ok(())}
    
    for path in paths {
        writer.write_all(path.as_bytes())?;
        writer.write_all(NEWLINE)?;
        if writer.buffer().len() >= FLUSH_THRESHOLD {writer.flush()?;}
    }
    

    writer.flush()?;


    Ok(())

}
