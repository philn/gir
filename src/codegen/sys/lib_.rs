use std::path::*;
use std::io::{Result, Write};

use env::Env;
use file_saver::*;
use library;
use nameutil::*;
use super::functions;
use super::statics;
use super::super::general;

pub fn generate(env: &Env) {
    println!("generating sys for {}", env.config.library_name);

    let path =  PathBuf::from(&env.config.target_path)
        .join(file_name_sys(&env.config.library_name, "lib"));
    println!("Generating file {:?}", path);

    save_to_file(path, &mut |w| generate_lib(w, env));
}

fn generate_lib<W: Write>(w: &mut W, env: &Env) -> Result<()>{
    try!(general::start_comments(w, &env.config));
    try!(statics::begin(w));

    let ns = env.library.namespace(library::MAIN_NAMESPACE);
    let classes = prepare_classes(ns);

    try!(generate_classes_structs(w, &classes));
    try!(generate_interfaces_structs(w, &prepare_interfaces(ns)));

    try!(statics::before_func(w));

    try!(writeln!(w, ""));
    try!(writeln!(w, "extern \"C\" {{"));
    try!(functions::generate_classes_funcs(w, env, &classes));

    //TODO: other functions
    try!(writeln!(w, "\n}}"));

    Ok(())
}

fn prepare_classes(ns: &library::Namespace) -> Vec<&library::Class> {
    let mut vec: Vec<&library::Class> = Vec::with_capacity(ns.types.len());
    for typ in &ns.types {
        if let &Some(library::Type::Class(ref klass)) = typ {
            vec.push(klass);
        }
    }
    vec.sort();
    vec
}

fn generate_classes_structs<W: Write>(w: &mut W, classes: &Vec<&library::Class>) -> Result<()> {
    try!(writeln!(w, ""));
    for klass in classes {
        try!(writeln!(w, "#[repr(C)]\npub struct {};", klass.glib_type_name));
    }

    Ok(())
}

fn prepare_interfaces(ns: &library::Namespace) -> Vec<&library::Interface> {
    let mut vec: Vec<&library::Interface> = Vec::with_capacity(ns.types.len());
    for typ in &ns.types {
        if let &Some(library::Type::Interface(ref interface)) = typ {
            vec.push(interface);
        }
    }
    vec.sort();
    vec
}

fn generate_interfaces_structs<W: Write>(w: &mut W, interfaces: &Vec<&library::Interface>) -> Result<()> {
    try!(writeln!(w, ""));
    for interface in interfaces {
        try!(writeln!(w, "#[repr(C)]\npub struct {};", interface.glib_type_name));
    }

    Ok(())
}
