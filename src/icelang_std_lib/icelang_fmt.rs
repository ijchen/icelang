use std::fmt::Write;

use crate::value::Value;

#[derive(Clone, Debug, Default)]
pub struct IcelangFmtArgs {
    pub debug: bool,
}

pub trait IcelangFmt {
    fn icelang_fmt(&self, buffer: &mut impl Write, fmt_args: &IcelangFmtArgs) -> std::fmt::Result;
}

impl IcelangFmt for Value {
    fn icelang_fmt(&self, buffer: &mut impl Write, fmt_args: &IcelangFmtArgs) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(buffer, "{value}"),
            Value::Byte(value) => write!(buffer, "{value:02X}"),
            Value::Float(value) => write!(buffer, "{value:?}"), // TODO implement correctly
            Value::Bool(value) => match value {
                true => write!(buffer, "true"),
                false => write!(buffer, "false"),
            },
            Value::String(value) => {
                if fmt_args.debug {
                    write!(buffer, "\"")?;
                    for ch in value.chars() {
                        match ch {
                            '"' => write!(buffer, "\\\"")?,
                            '\\' => write!(buffer, "\\\\")?,
                            '\t' => write!(buffer, "\\t")?,
                            '\n' => write!(buffer, "\\n")?,
                            '\r' => write!(buffer, "\\r")?,
                            '\0' => write!(buffer, "\\0")?,
                            '\x20'..='\x7E' => write!(buffer, "{ch}")?,
                            '\x01'..='\x1F' | '\x7F' => {
                                write!(buffer, "\\x{:02X}", ch as u8)?;
                            }
                            '\u{80}'.. => {
                                write!(buffer, "\\u{{{:X}}}", ch as u32)?;
                            }
                        }
                    }
                    write!(buffer, "\"")?;

                    Ok(())
                } else {
                    write!(buffer, "{value}")
                }
            }
            Value::List(_) => todo!(),
            Value::Dict(_) => todo!(),
            Value::Null => write!(buffer, "null"),
        }
    }
}
