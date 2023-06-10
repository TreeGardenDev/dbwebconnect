use crate::Data2;
use crate::Reader;
use crate::dbconnect;
use crate::pushdata;
pub fn read_csv2(file: &String, tablename:&String, database:&String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    
    let mut combinedcol=Vec::new();
    let mut data: Vec<Data2> = Vec::new();
    //let mut data2 : Vec<InsertData>=Vec::new();
    let mut vecty:Vec<&str>=Vec::new();
    //iterate through every column in csv file
    //Do not iterate through first roq
    
    let mut rdr=Reader::from_path(file)?;
    println!("Headers: {:?}",rdr.headers()?);
        for result in rdr.records() {
        let record = result?;
        //get first row of csv file 
        //get first column of csv file
        println!("{:?}", record);
        let mut columnvector=Vec::new();
        for column in 0..record.len(){
            columnvector.push(record[column].to_string());
       // }?
        
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
       // println!("New Struct below");
       // println!("{:?}", data);
       // println!("New Struct above");
    let mut rdr3=Reader::from_path(file)?;
        let columnname2 = rdr3.headers()?;
        for u in columnname2{

            vecty.push(&u);
        }
   //let db:&str=&(*database); 
  // println!("Database: {}", db);
  // println!("Table: {}", tablename);
   // println!("{:?}", data);
    //let tablename= std::env::args().nth(2).expect("No Table");
    let connection = dbconnect::internalqueryconn();
    let _ = pushdata::execute_insert2(data, tablename,connection, database.to_string());
    Ok(())
}
