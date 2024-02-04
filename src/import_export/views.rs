use std::error::Error;
use csv::Writer;
use crate::{
    models::{
        ListUser
    },
};


pub async fn write_to_csv(
    data: Vec<ListUser>,
) -> Result<(), Box<dyn Error>> {

    let mut wtr = Writer::from_writer(vec![]);

    for pat in data {
        wtr.serialize(pat)?;
    }
    
    wtr.flush()?;

    Ok(())
}
