use mysql::prelude::*;
use mysql::*;
//read from csv file to create table in mariadb with given column names

pub fn exec_statement(conn: &mut PooledConn, statement: &str) {
    conn.query_drop(statement).unwrap();
}
pub fn create_table(conn: &mut PooledConn,database:&str, table_name: &str, column_names: &Vec<String>, column_types: &Vec<String> ){
    let mut query = String::from("CREATE TABLE ");

    query.push_str(database);
    query.push_str(".");
    query.push_str(table_name);
    query.push_str(" (");
    for i in 0..column_names.len() {
        query.push_str(column_names[i].as_str());
        query.push_str(" ");
        query.push_str(column_types[i].as_str());
        query.push_str(", ");

    }
    
    query.pop();
    query.pop();
    query.push_str(")");
    println!("{}", query);
    conn.query_drop(query).unwrap();
}
pub fn create_table_web(database:&str,table_name: &str, column_names: &Vec<(String,String)>, column_types: &Vec<(String, String)> )->String{
    let mut query = String::from("CREATE TABLE ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(table_name);
    query.push_str(" (");
    query.push_str("INTERNAL_PRIMARY_KEY INT NOT NULL AUTO_INCREMENT PRIMARY KEY, ");
    for i in 0..column_names.len() {
        let valid=validate_unprotected_term(column_names[i].0.as_str());
        if valid.0==false{
            println!("Invalid column name: {}",column_names[i].0);
            let mut error=String::from("Invalid column name: ");
            error.push_str(column_names[i].0.as_str());
            return error;
        }
        query.push_str(column_names[i].1.as_str());
        query.push_str(" ");
        query.push_str(column_types[i].1.as_str());
        //grab first 7 characters of column type
        //

        if column_types[i].1.get(0..7)==Some("VARCHAR"){
            query.push_str(" DEFAULT \"\" ");
        }
        if column_types[i].1.get(0..3)==Some("INT"){
            query.push_str(" DEFAULT 0 ");
        }

        query.push_str(", ");
    }
    query.pop();
    query.pop();
    query.push_str(")");
    println!("{}", query);
    query
}

//pub fn create_table_web_gps(database:&str,table_name: &str, column_names: &Vec<(String,String)>, column_types: &Vec<(String, String)> )->String{
pub fn create_table_web_gps(database:&str,table_name: &str )->String{
    let mut query = String::from("CREATE TABLE ");
    query.push_str(database);
    query.push_str(".");
    query.push_str(table_name);
    query.push_str("_GPS");
    query.push_str(" (");
    query.push_str("INTERNAL_PRIMARY_KEY INT NOT NULL AUTO_INCREMENT PRIMARY KEY, ");
    query.push_str("MAIN_TABLE_ID INT, ");
    query.push_str("GPS_ID INT, ");
    query.push_str("X_COORD VARCHAR(100), ");
    query.push_str("Y_COORD VARCHAR(100), ");
    query.push_str("Attachment BLOB, ");
    //for i in 0..column_names.len() {
    //    let valid=validate_unprotected_term(column_names[i].1.as_str());
    //    if valid.0==false{
    //        println!("Invalid column name: {}",valid.1);
    //        let mut error=String::from("Invalid column name: ");
    //        error.push_str(column_names[i].1.as_str());
    //        return error;
    //    }
    //    query.push_str(column_names[i].1.as_str());
    //    query.push_str(" ");
    //    query.push_str(column_types[i].1.as_str());
    //    query.push_str(", ");
    //}
    query.pop();
    query.pop();
    query.push_str(")");
    println!("{}", query);
    query
}
pub fn parse_json(json: Vec<(String, String)>)-> (Vec<(String,String)>, Vec<(String,String)>){
    let mut columnstr=json[0].1.clone();
    let mut datatypestr=json[1].1.clone();
    columnstr=columnstr.replace("\"","");
    datatypestr=datatypestr.replace("\"","");
    columnstr=columnstr.replace("[","");
    datatypestr=datatypestr.replace("[","");
    columnstr=columnstr.replace("]","");
    datatypestr=datatypestr.replace("]","");
    columnstr=columnstr.replace(" ","");
    datatypestr=datatypestr.replace(" ","");
    columnstr=columnstr.replace("\n","");
    datatypestr=datatypestr.replace("\n","");
    columnstr=columnstr.replace("\r","");
    datatypestr=datatypestr.replace("\r","");
    columnstr=columnstr.replace("\t","");
    datatypestr=datatypestr.replace("\t","");
    columnstr=columnstr.replace("\r\n","");
    datatypestr=datatypestr.replace("\r\n","");
    columnstr=columnstr.replace("{","");
    datatypestr=datatypestr.replace("{","");
    columnstr=columnstr.replace("}","");
    datatypestr=datatypestr.replace("}","");

    let column=columnstr.split(",");
    let datatype=datatypestr.split(",");
    let column:Vec<String>=column.map(|s| s.to_string()).collect();
    let datatype:Vec<String>=datatype.map(|s| s.to_string()).collect();
    //split string in vector by colon
    //push into vector
    let mut splitcolumn:Vec<(String,String)>=Vec::new();
    let mut splitdatatype:Vec<(String,String)>=Vec::new();
    for i in 0..column.len(){
        let split=column[i].split(":");
        let split2=datatype[i].split(":");
        let splitvec:Vec<&str>=split.collect();
        let splitvec2:Vec<&str>=split2.collect();
        splitcolumn.push((splitvec[0].to_string(),splitvec[1].to_string()));
        splitdatatype.push((splitvec2[0].to_string(),splitvec2[1].to_string()));
    }

    (splitcolumn,splitdatatype)

}

pub fn validate_unprotected_term(column:&str)->(bool,&str){

    //glet protected=["PRIMARY_KEY","INTERNAL_PRIMARY_KEY","GPS_ID","X_COORD","Y_COORD","Attachment"];
    //include list of sql reserved words
    //include list of sql data types
    let reserved=["ADD","ALL","ALTER","ANALYZE","AND","AS","ASC","ASENSITIVE","BEFORE","BETWEEN","BIGINT","BINARY","BLOB","BOTH","BY","CALL","CASCADE","CASE","CHANGE","CHAR","CHARACTER","CHECK","COLLATE","COLUMN","CONDITION","CONSTRAINT","CONTINUE","CONVERT","CREATE","CROSS","CURRENT_DATE","CURRENT_TIME","CURRENT_TIMESTAMP","CURRENT_USER","CURSOR","DATABASE","DATABASES","DAY_HOUR","DAY_MICROSECOND","DAY_MINUTE","DAY_SECOND","DEC","DECIMAL","DECLARE","DEFAULT","DELAYED","DELETE","DESC","DESCRIBE","DETERMINISTIC","DISTINCT","DISTINCTROW","DIV","DOUBLE","DROP","DUAL","EACH","ELSE","ELSEIF","ENCLOSED","ESCAPED","EXISTS","EXIT","EXPLAIN","FALSE","FETCH","FLOAT","FLOAT4","FLOAT8","FOR","FORCE","FOREIGN","FROM","FULLTEXT","GOTO","GRANT","GROUP","HAVING","HIGH_PRIORITY","HOUR_MICROSECOND","HOUR_MINUTE","HOUR_SECOND","IF","IGNORE","IN","INDEX","INFILE","INNER","INOUT","INSENSITIVE","INSERT","INT","INT1","INT2","INT3","INT4","INT8","INTEGER","INTERVAL","INTO","IS","ITERATE","JOIN","KEY","KEYS","KILL","LEADING","LEAVE","LEFT","LIKE","LIMIT","LINEAR","LINES","LOAD","LOCALTIME","LOCALTIMESTAMP","LOCK","LONG","LONGBLOB","LONGTEXT","LOOP","LOW_PRIORITY","MASTER_BIND","MASTER_SSL_VERIFY_SERVER_CERT","MATCH","MAXVALUE","MEDIUMBLOB","MEDIUMINT","MEDIUMTEXT","MIDDLEINT","MINUTE_MICROSECOND","MINUTE_SECOND","MOD","MODIFIES","NATURAL","NOT","NO_WRITE_TO_BINLOG","NULL","NUMERIC","ON","OPTIMIZE","OPTION","OPTIONALLY","OR","ORDER","OUT","OUTER","OUTFILE","PRECISION","PRIMARY","PROCEDURE","PURGE","RANGE","READ","READS","READ_WRITE","REAL","REFERENCES","REGEXP","RELEASE","RENAME","REPEAT","REPLACE","REQUIRE","RESIGNAL","RESTRICT","RETURN","REVOKE","RIGHT","RLIKE","SCHEMA","SCHEMAS","SECOND_MICROSECOND","SELECT","SENSITIVE"];
    let datatypes=["INT","VARCHAR","CHAR","TEXT","DATE","TIME","DATETIME","TIMESTAMP","FLOAT","DOUBLE","DECIMAL","BINARY","VARBINARY","TINYBLOB","BLOB","MEDIUMBLOB","LONGBLOB","TINYTEXT","TEXT","MEDIUMTEXT","LONGTEXT","ENUM","SET"];
    let mut protected:Vec<&str>=Vec::new();
    protected.push("PRIMARY_KEY");
    protected.push("INTERNAL_PRIMARY_KEY");
    protected.push("GPS_ID");
    protected.push("X_COORD");
    protected.push("Y_COORD");
    protected.push("Attachment");
    for i in 0..reserved.len(){
        if column.to_uppercase()==reserved[i]{
            return (false, column)
        }
    }
    for i in 0..datatypes.len(){
        if column.to_uppercase()==datatypes[i]{
            return (false, column)
        }
    }
    for i in 0..protected.len(){
        if column.to_uppercase()==protected[i]{
            return (false, column)
        }
    }
    //
    (true, column)
}
