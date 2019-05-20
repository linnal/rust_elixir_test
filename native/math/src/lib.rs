#[macro_use] extern crate rustler;

use rustler::{Env, Term, NifResult, Encoder};

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

rustler_export_nifs! {
    "Elixir.Math",
    [
    ("add", 2, add),
    ("sum_list", 1, sum_list),
    ("append_to_list", 2, append_to_list)
    ],
    None // if you need to do additional things when the NIF is loaded
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn sum_list<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let ls: Vec<i64> = args[0].decode()?;

    let result: i64 = ls.iter().sum();

    Ok((atoms::ok(), result).encode(env))
}


fn append_to_list<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let mut ls: Vec<i64> = args[0].decode()?;
    let elem: i64 = args[1].decode()?;

    ls.push(elem);

    Ok((atoms::ok(), ls).encode(env))
}