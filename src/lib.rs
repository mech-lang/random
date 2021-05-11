extern crate mech_core;
extern crate mech_utilities;
#[macro_use]
extern crate lazy_static;
extern crate oorandom;
use mech_core::{Transaction};
use mech_core::{Value, ValueMethods, IndexIterator, Table, ValueIterator};
use mech_core::{Quantity, ToQuantity, QuantityMath, hash_string};
use oorandom::Rand64;
use std::sync::Mutex;

static DEFAULT_SEED: u64 = 0x242689ed11c79076;

lazy_static! {
  static ref ROWS: u64 = hash_string("rows");
  static ref COLUMNS: u64 = hash_string("columns");
  static ref RNG: Mutex<Rand64> = Mutex::new(Rand64::new(DEFAULT_SEED.into()));
}

#[no_mangle]
pub extern "C" fn random_uniform(arguments: &Vec<(u64, ValueIterator)>) {  
  let (in_arg_name1, rows) = &arguments[0];
  let (in_arg_name2, columns) = &arguments[1];
  let (_, mut out) = arguments.last().unwrap().clone();
  if *in_arg_name1 == *ROWS && *in_arg_name2 == *COLUMNS {
    for ((rows_value,_),(columns_value,_)) in rows.clone().zip(columns.clone()) {
      match (rows_value.as_u64(), columns_value.as_u64()) {
        (Some(rows_u64),Some(columns_u64)) => {
          out.resize(rows_u64 as usize, columns_u64 as usize);
          let mut rng = RNG.lock().unwrap();
          for out_ix in out.linear_index_iterator() {
            let mut random_value = rng.rand_float();
            out.set_unchecked_linear(out_ix, Value::from_f64(random_value));
          }
        }
        _ => (),
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}

#[no_mangle]
pub extern "C" fn random_uniform__integer(arguments: &Vec<(u64, ValueIterator)>) {  
  let (in_arg_name1, rows) = &arguments[0];
  let (in_arg_name2, columns) = &arguments[1];
  let (_, mut out) = arguments.last().unwrap().clone();
  if *in_arg_name1 == *ROWS && *in_arg_name2 == *COLUMNS {
    for ((rows_value,_),(columns_value,_)) in rows.clone().zip(columns.clone()) {
      match (rows_value.as_u64(), columns_value.as_u64()) {
        (Some(rows_u64),Some(columns_u64)) => {
          out.resize(rows_u64 as usize, columns_u64 as usize);
          let mut rng = RNG.lock().unwrap();
          for out_ix in out.linear_index_iterator() {
            let mut random_value = rng.rand_u64();
            out.set_unchecked_linear(out_ix, Value::from_u64(random_value));
          }
        }
        _ => (),
      }
    }
  } else {
    // TODO Warn about unknown argument
  }
}