use anyhow::Result;
use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use std::fs::read_to_string;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

#[derive[Default]]
pub struct CsvReaderImpl<T> {
    phantom: PhantomData<T>;
}

impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let string_data = read_to_string(path_buf)?;
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(string_data.as_bytes());
        let rows = reader.deserialize::<T>();
        let mut result = Vec::<T>::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}

#[derive[Default]]
pub struct JsonReaderImpl<T> {
    phantom: PhantomData<T>;
}

impl<T> JsonReader<T> for JsonReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let buf_reader = File::open(path_buf).map(|file| BufReader::new(file))?;
        let result = serde_json::from_reader(buf_reader)?;
        Ok(result)
    }
}
