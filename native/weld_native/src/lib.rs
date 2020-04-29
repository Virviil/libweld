#[macro_use]
extern crate rustler;

use rustler::{Encoder, Env, Error, Term, ResourceArc, SchedulerFlags};
use std::sync::{Arc, Mutex, RwLock};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom nil;
        atom bad_reference;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

fn on_init<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    // Unsafe data types
    resource_struct_init!(WeldContext, env);
    resource_struct_init!(WeldValue, env);

    // // Safe data types
    resource_struct_init!(WeldConf, env);
    resource_struct_init!(WeldModule, env);
    resource_struct_init!(WeldType, env);
    true
}

rustler::rustler_export_nifs! {
    "Elixir.Weld.Native",
    [
        ("weld_module_compile", 2, weld_module_compile, SchedulerFlags::DirtyCpu),
        ("weld_conf_new", 0, weld_conf_new),
        ("weld_conf_get", 2, weld_conf_get),
        ("weld_conf_set", 3, weld_conf_set),
        ("weld_module_run", 3, weld_module_run),
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

struct WeldValue {
    value: RwLock<weld::WeldValue>,
}

impl WeldValue {
    fn from_weld(value: weld::WeldValue) -> WeldValue {
        WeldValue {
            value: RwLock::new(value)
        }
    }
}
unsafe impl std::marker::Send for WeldValue{}
unsafe impl std::marker::Sync for WeldValue{}


// Safe
struct WeldConf {
    conf: RwLock<weld::WeldConf>,
}

struct WeldType {
    ty: Arc<Mutex<weld::ast::Type>>,
}

struct WeldModule {
    module: RwLock<weld::WeldModule>,
}

// Module API
fn weld_module_compile<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let code: String = args[0].decode()?;
    let conf_container: ResourceArc<WeldConf> = args[1].decode()?;

    let conf = conf_container.conf.read().unwrap();

    match weld::WeldModule::compile(code, &conf) {
        Ok(compiled_module) => {
            let module_container = WeldModule {
                module: RwLock::new(compiled_module)
            };
            Ok((atoms::ok(), ResourceArc::new(module_container)).encode(env))
        },
        Err(reason) => {
            Ok((atoms::error(), reason.message().to_str().unwrap()).encode(env))
        }
    }
}

fn weld_module_run<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let module_container: ResourceArc<WeldModule> = args[0].decode()?;
    let conf_container: ResourceArc<WeldConf> = args[1].decode()?;
    let input_container: ResourceArc<WeldValue> = args[2].decode()?;

    let module = module_container.module.read().unwrap();
    let conf = conf_container.conf.read().unwrap();
    let input = input_container.value.read().unwrap();

    let context = &mut weld::WeldContext::new(&conf).unwrap();

    let result = unsafe {
        module.run(context, &input)
    };

    match result {
        Ok(value) => {
            Ok(
                (
                    atoms::ok(),
                    ResourceArc::new(WeldValue::from_weld(value))
                ).encode(env)
            )
        }
        Err(reason) => {
            Ok((atoms::error(), reason.message().to_str().unwrap()).encode(env))
        }
    }
}

// Type API
fn weld_value_new<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    Ok(atoms::ok().encode(env))
}

// Configuration API
fn weld_conf_new<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let conf = WeldConf {
        conf: RwLock::new(weld::WeldConf::new())
    };
    Ok(ResourceArc::new(conf).encode(env))
}

fn weld_conf_get<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let container: ResourceArc<WeldConf> = args[0].decode()?;
    let key: String = args[1].decode()?;

    let conf = container.conf.read().unwrap();

    match conf.get(&key) {
        Some(value) => Ok(value.to_str().unwrap().encode(env)),
        None => Ok(atoms::nil().encode(env))
    }
}

fn weld_conf_set<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let container: ResourceArc<WeldConf> = args[0].decode()?;
    let key: String = args[1].decode()?;
    let value: String = args[2].decode()?;

    let mut conf = container.conf.write().unwrap();
    conf.set(key, value);
    Ok(atoms::ok().encode(env))
}