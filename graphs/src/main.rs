use polars::prelude::*;
// use std::{fs::File, io::Read, vec};
fn main() {
    let path= PlPath::new("input/dataset_algo.csv");
    // let mut file= File::open("input/dataset_algo.xlsx").expect("error reading the file");
    // let mut data=vec![];
    
    let df= LazyCsvReader::new(path).finish().unwrap().collect().unwrap();
    let data: Vec<_>= df.get_columns()[0].i64().unwrap().iter().zip(df.get_columns()[1].i64().unwrap().iter()).map(|(x,y)| (x.unwrap(), y.unwrap())).collect();
    println!("{:?}",data); 
    // let dataset=file.read_to_end(&mut data);
    //if the read is successsful this function will return the total number of rows being read
}
