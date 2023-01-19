use crate::value::Value;

#[derive(Clone, Debug, Default)]
pub struct IcelangFmtArgs {
    pub debug: bool,
}

pub trait IcelangFmt {
    fn icelang_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        fmt_args: &IcelangFmtArgs,
    ) -> std::fmt::Result;
}

impl IcelangFmt for Value {
    fn icelang_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        fmt_args: &IcelangFmtArgs,
    ) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Byte(value) => write!(f, "{value:02X}"),
            Value::Float(_) => todo!(),
            Value::Bool(value) => match value {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::String(value) => {
                if fmt_args.debug {
                    todo!()
                } else {
                    write!(f, "{value}")
                }
            }
            Value::List(_) => todo!(),
            Value::Dict(_) => todo!(),
            Value::Null => write!(f, "null"),
        }
    }
}
