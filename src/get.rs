use std::error::Error;

use clap::Parser;
use toml_edit::{Document, Item, Key, Value};

#[derive(Parser, Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn run(&self, doc: &Document) -> Result<(), Box<dyn Error>> {
        let keys = Key::parse(&self.key)?;
        let mut item = doc.as_item();
        for key in keys {
            match item.get(key.to_string()) {
                Some(inner) => item = inner,
                None => panic!("not found"),
            }
        }
        match item {
            Item::None => (),
            Item::Value(v) => match v {
                Value::String(s) => println!("{}", s.value()),
                Value::Integer(i) => println!("{}", i.value()),
                Value::Float(_) => todo!(),
                Value::Boolean(_) => todo!(),
                Value::Datetime(_) => todo!(),
                Value::Array(_) => todo!(),
                Value::InlineTable(_) => todo!(),
            },
            Item::Table(_) => todo!(),
            Item::ArrayOfTables(_) => todo!(),
        }
        Ok(())
    }
}
