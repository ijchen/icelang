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
            Value::Float(_) => todo!(),
            Value::Bool(value) => match value {
                true => write!(buffer, "true"),
                false => write!(buffer, "false"),
            },
            Value::String(value) => {
                if fmt_args.debug {
                    todo!()
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
