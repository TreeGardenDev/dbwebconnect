use crate::ColData;
use crate::Reader;


pub fn read_fields(file: &String) -> Vec<String>{
    //fn read_csv() ->Vec<Data> {
    
    let mut rdr = Reader::from_path(file);
    let mut data: Vec<String> = Vec::new();
    //let new_data: ColData = ColData::new();
    for result in rdr.expect("Reason").records() {
        let record = result;
        data.push(record.expect("Reason").get(0).expect("Reason").to_string());
    }
//    println!("{:?}", data);
    return data

}

