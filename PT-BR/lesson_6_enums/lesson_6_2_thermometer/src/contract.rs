use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};


#[allow(unused_imports)]
use near_sdk::{
    collections::Vector,
    near_bindgen,
    BorshStorageKey,
};


near_sdk::setup_alloc!();


use crate::{
    temperature::temp_format::TempFormat,
    utils::log,
    entry::Entry,
};


#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Entry,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    temp_format: TempFormat,
    entries: Vector<Entry>,
    temp_length: u32,
}


impl Default for Contract {
    fn default() -> Self {
        // let temp_format = format!("{}", TempFormat::default());
        let temp_format = TempFormat::default();
        let entries = Vector::new(StorageKey::Entry);

        Contract{
            temp_format,
            entries,
            temp_length: 0,
        }
    }
}


#[near_bindgen]
impl Contract{
    pub fn set_format(&mut self, temp_format: String) {
        log("Called set_format");
        // let temp_format = String::from(TempFormat::from(temp_format));
        let temp_format = TempFormat::new(&temp_format);

        log(
            &format!("Setting default temperature format to {}", &temp_format)
        );

        self.temp_format = temp_format;
    }

    // time: (u8, u8, u8, f32),
    // date: (i32, String, u8),
    // temp_format: &TempFormat, 
    // value: f32, 
    // arg_temp: Option<String>,
    
    // Exemplo de argumento para esta função: '{"time": [11, 32, 10, 0.85], "date": [2022, "feb", 11], "value": 127, "arg_temp": "k" }'
    pub fn new_entry(
        &mut self, 
        time: (u8, u8, u8, f32),
        date: (i32, String, u8),
        value: f32, 
        arg_temp: Option<String>,
    ){
        // log("Creating TempFormat");
        // let temp_format: TempFormat = TempFormat::new(&temp_format);

        log("Creating Entry");
        let entry: Entry = Entry::new(time, date, &self.temp_format, value, arg_temp);

        log("Pushing entry to Vector");
        self.entries.push(&entry);
    }

    pub fn get_format(&self) -> String {
        let temp_format: String = String::from(&self.temp_format);
        
        temp_format
    }

    // pub fn new_entry(&mut self, )
    pub fn list_entries(&self) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::with_capacity(
            self.entries.len()
                .try_into()
                .unwrap()
        );

        for entry in self.entries.iter(){
            entries.push(entry)
        }

        entries
    }
}

