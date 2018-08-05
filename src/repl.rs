use std::io::{BufRead, Write, Result};
use lexer::Lexer;
use token::Token;

const PROMPT: &'static [u8] = b">> ";

pub fn start<R, W>(mut reader: R, mut writer: W) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    loop {
        writer.write(PROMPT)?;
        writer.flush()?;
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            break;
        }

        let mut lxr = Lexer::new(&line);

        loop {
            let token = lxr.next_token();
            if token == Token::EoF {
                break;
            } else {
                writer.write_fmt(format_args!("{}\n", token))?;
            }
        }
    }

    Ok(())
}
