use crate::Reader;

pub fn read_fields(file: &String) -> Vec<String> {
    
    //fn read_csv() ->Vec<Data> {

    let rdr = Reader::from_path(file);
    let mut data: Vec<String> = Vec::new();
    //let new_data: ColData = ColData::new();
    data.push("INTERNAL_PRIMARY_KEY".to_string());
    for result in rdr.expect("Reason").records() {
        let record = result;
        data.push(record.expect("Reason").get(0).expect("Reason").to_string());
    }
    //    println!("{:?}", data);
    return data;
}

pub fn read_types(file: &String) -> Vec<String> {
    //fn read_csv() ->Vec<Data> {

    let rdr = Reader::from_path(file);
    let mut data: Vec<String> = Vec::new();
    data.push("int primary key not null auto_increment".to_string());
    //let new_data: ColData = ColData::new();
    for result in rdr.expect("Reason").records() {
        let record = result;
        data.push(record.expect("Reason").get(1).expect("Reason").to_string());
    }
    println!("Types:");
    println!("{:?}", data);
    return data;
}
