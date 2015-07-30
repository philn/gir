use std::collections::HashSet;

use analysis::rust_type::*;
use env::Env;
use library;

pub struct Info {
    pub parameter: Option<library::Parameter>,
    pub commented: bool,
}

pub fn analyze(env: &Env, type_: &library::Function,
    used_types: &mut HashSet<String>) -> Info {

    let parameter = if type_.ret.typ == Default::default() { None } else {
        used_rust_type(env, type_.ret.typ).ok().map(|s| used_types.insert(s));
        Some(library::Parameter {
                //Many missing return nullables in girs so detecting it
                nullable: type_.ret.nullable ||
                    can_be_nullable_return(env, type_.ret.typ),
                ..type_.ret.clone()
            })
    };
    let commented = if type_.ret.typ == Default::default() { false } else {
        parameter_rust_type(env, type_.ret.typ, type_.ret.direction).is_err()
    };

    Info {
        parameter: parameter,
        commented: commented,
    }
}

fn can_be_nullable_return(env: &Env, type_id: library::TypeId) -> bool
{
    use library::Type::*;
    use library::Fundamental::*;
    match env.library.type_(type_id) {
        &Fundamental(fund) => match fund {
            Pointer => true,
            Utf8 => true,
            Filename => true,
            _ => false,
        },
        &Alias(ref alias) => can_be_nullable_return(env, alias.typ),
        &Enumeration(_) => false,
        &Bitfield(_) => false,
        &Record(_) => false,
        &Union(_) => false,
        &Function(_) => true,
        &Interface(_) => true,
        &Class(_) => true,
        _ => true
    }
}
