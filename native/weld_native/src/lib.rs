#[macro_use]
extern crate rustler;

use rustler::{Encoder, Env, Error, Term, TermType, ResourceArc};
use std::sync::{Arc, Mutex, RwLock};


mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

fn on_init<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    // Unsafe data types
    resource_struct_init!(WeldContext, env);
    resource_struct_init!(WeldType, env);

    // Safe data types
    resource_struct_init!(WeldConf, env);
    resource_struct_init!(WeldModule, env);
    resource_struct_init!(WeldValue, env);
    true
}

rustler::rustler_export_nifs! {
    "Elixir.Weld.Native",
    [
        ("add", 2, add)
    ],
    Some(on_init)
}

// Part with data structure descriptions

// Unsafe
struct WeldContext {
    context: Arc<Mutex<weld::WeldContext>>,
}
unsafe impl std::marker::Send for WeldContext{}
unsafe impl std::marker::Sync for WeldContext{}

struct WeldType {
    ty: Arc<Mutex<weld::ast::Type>>,
}
unsafe impl std::marker::Send for WeldValue{}
unsafe impl std::marker::Sync for WeldValue{}


// Safe
struct WeldConf {
    conf: RwLock<weld::WeldConf>,
}

struct WeldValue {
    value: RwLock<weld::WeldValue>,
}

struct WeldModule {
    module: RwLock<weld::WeldModule>,
}


fn weld_conf_new<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let conf = WeldConf {
        conf: RwLock::new(weld::WeldConf::new())
    };
    Ok(ResourceArc::new(conf).encode(env))
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}
