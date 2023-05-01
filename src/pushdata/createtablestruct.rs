use crate::Data2;
use crate::Reader;
use crate::pushdata;
pub fn read_csv2(file: &String, tablename:String, database:&String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let mut combinedcol=Vec::new();
    let mut data: Vec<Data2> = Vec::new();
    //let mut data2 : Vec<InsertData>=Vec::new();
    let mut vecty:Vec<&str>=Vec::new();
    //iterate through every column in csv file
    //Do not iterate through first roq
    let mut tablename2=String::from("");
    let mut database2=String::from("");
    let mut rdr=Reader::from_path(file)?;
    
        for result in rdr.records() {
        let record = result?;

        println!("{:?}", record);
        let mut columnvector=Vec::new();
        for column in 0..record.len(){

            columnvector.push(record[column].to_string());
       // }
         //   else{
         //       database2=record[0].to_string();
         //       tablename2=record[1].to_string();
        //    }
        }
        combinedcol.push(columnvector);

    }
        data.push(Data2 {
             columns: combinedcol,
         });
        println!("New Struct below");
        println!("{:?}", data);
        println!("New Struct above");
    let mut rdr3=Reader::from_path(file)?;
        let columnname2 = rdr3.headers()?;
        for u in columnname2{

            vecty.push(&u);
        }
    for i in 0..data.len(){
        for j in 0..data[i].columns.len(){
            println!("New Column");
            for k in 0..data[i].columns[j].len(){
                //println!("Data below");
                println!("{:?}", data[i].columns[j][k]);
                //println!("Data above");
            }
        }
    }
   //let db:&str=&(*database); 
  // println!("Database: {}", db);
  // println!("Table: {}", tablename);
   // println!("{:?}", data);
    //let tablename= std::env::args().nth(2).expect("No Table");
    let connection = crate::dbconnect::database_connection(database);
    let _ = pushdata::execute_insert2(data, tablename,connection, database.to_string());
    Ok(())
}
