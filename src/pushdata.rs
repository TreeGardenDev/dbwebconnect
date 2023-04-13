use csv::StringRecord; use crate::Data2;
//use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;
use crate::Table;
pub mod gettablecol;
pub mod createtablestruct;
#[derive(Debug)] struct InsertData<'a>{
    data: Vec<&'a str>,
}

//fn execute_insert(
//    data: Vec<Data>,
//    //data: &Vec<String>,
//    tablename: String,
//    mut conn: PooledConn,
//    columnames: Vec<&str>,
//) -> std::result::Result<(), Box<dyn std::error::Error>> {
//    
//    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename).unwrap();
//    println!("{:?}", columname);
//    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
//    println!("{}", insertstatement);
//
//    
//     conn.exec_batch(
//        r"INSERT INTO Data(id, name, age, address, salary)
//       VALUES (:id, :name, :age, :address, :salary)",
//       data.iter().map(|p| {
//            params! {
//                "id" => p.id,
//                "name" => &p.name,
//                "age" => p.age,
//                "address" => &p.address,
//                "salary" => p.salary,
//            }
//        }),
//    )?;
//    //insert into mysql data from data variable into columns in columnname variable
//
//   // conn.exec_batch(
//    //   insertstatement, 
//        
//      //  data.iter().map(|p| {
//    //  data.chunks(columnname.len()).map(|p|{
//            //let
////                //let mut
//     //       params! {
//    //for i in columnname.iter(){
////   //             for i in &columnname{
//     //           i=>  p.iter().next().unwrap(),
////                }   
////            }
////        }),
////   )?;
//
//    Ok(())
//    //todo
//}

fn execute_insert2(
    data: Vec<Data2>,
    //data: &Vec<String>,
    tablename: String,
    mut conn: PooledConn,
    columnames: Vec<&str>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    let columname: Vec<String> = gettablecol::get_table_col(&mut conn, &tablename).unwrap();
    println!("{:?}", columname);
    let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename, data);
    println!("{}", insertstatement);

   // let mut params = Vec::new();
   // for i in 0..data.len(){
   //     for j in 0..data[i].columns.len(){
   //         for k in 0..data[i].columns[j].len(){
   //             let datarecord=data[i].columns[j][k];
   //             params.push(params!{
   //                 "id" => datarecord.id,
   //                 "name" => &datarecord.name,
   //                 "age" => datarecord.age,
   //                 "address" => &datarecord.address,
   //                 "salary" => datarecord.salary,
   //             });
   //         }
   //     }
   // }
    //run inser statement
    
    conn.query_drop(insertstatement)?;
 //    conn.exec_batch(
 //       r"INSERT INTO Data(id, name, age, address, salary)
 //      VALUES (:id, :name, :age, :address, :salary)",
 //      data.columns.iter().map(|p| {
 //           params! {
 //               "id" => p.id,
 //               "name" => &p.name,
 //               "age" => p.age,
 //               "address" => &p.address,
 //               "salary" => p.salary,
 //           }
 //       }),
 //   )?;
    //insert into mysql data from data variable into columns in columnname variable
 //   println!("In exc function");
 //   for i in 0..data.len(){
 //       for j in 0..data[i].columns.len(){
 //           println!("New Column");
 //           for k in 0..data[i].columns[j].len(){
 //               //println!("Data below");
 //               println!("{:?}", data[i].columns[j][k]);
 //               //println!("Data above");
 //               let datarecord=data[i].columns[j][k];
 //               //insert into mysql data from data variable into columns in columnname variable
 //               //let insertstatement =gettablecol::createinsertstatement(&mut conn, &tablename);
 //               //println!("{}", insertstatement);
 //               
 //   

 //           }
 //       }
 //   }

   // conn.exec_batch(
    //   insertstatement, 
        
      //  data.iter().map(|p| {
    //  data.chunks(columnname.len()).map(|p|{
            //let
//                //let mut
     //       params! {
    //for i in columnname.iter(){
//   //             for i in &columnname{
     //           i=>  p.iter().next().unwrap(),
//                }   
//            }
//        }),
//   )?;

    Ok(())
    //todo
}
