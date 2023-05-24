// Purpose: generate form for table
use uuid::Uuid;
use std::io::Write;
use futures_util::{TryStreamExt as _, TryStream};
use actix_multipart::Multipart;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{web, Error, HttpResponse};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
    pub database: Text<String>,
    pub table: Text<String>,
}
#[derive(Debug, MultipartForm)]
pub struct CreateTable{
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
    pub database: Text<String>,
    pub table: Text<String>,
}

#[derive(Debug, MultipartForm)]
pub struct CreateRelation{
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
    pub database: Text<String>,
}


//use file from Insert form in file_upload2
pub fn getCreateRelationship()->String{
    let html = r#"<html>
        <style>
    body {
    background-color: lightblue;
    font-family: 'Roboto', sans-serif;
    font-weight: 300;
    font-size: 14px;
    color: #666666;
    -webkit-font-smoothing: antialiased;
    -webkit-text-size-adjust: 100%;
    -ms-text-size-adjust: 100%;
    text-size-adjust: 100%;
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    overflow: hidden;
    text-align: center;
}
form {
    background: #fff;
    padding: 40px;
    max-width: 600px;
    margin: 40px auto;
    border-radius: 4px;
    box-shadow: 0 4px 10px 4px rgba(19, 35, 47, 0.3);
}
input {
    width: 100%;
    padding: 12px 20px;
    margin: 8px 0;
    box-sizing: border-box;
}
    </style>
        <head>
            <title>Create Relationship</title>
        </head>
        <body>
            <br><br>
            <h1>Create Relationship</h1>
            <form action="/createrelation" method="post" enctype="multipart/form-data">
                <label for="file">File:</label>
                <input type="file" id="file" name="file"><br><br>
                <label for="database">Database:</label>
                <input type="text" id="database" name="database"><br><br>
                <input type="submit" value="Submit">
            </form>
        </body>
    </html>"#;
    html.to_string()
}

pub fn getcreaterelationshipdefined()->String{

    let html = r#"<html>
        <style>
    body {
    background-color: lightblue;
    font-family: 'Roboto', sans-serif;
    font-weight: 300;
    font-size: 14px;
    color: #666666;
    -webkit-font-smoothing: antialiased;
    -webkit-text-size-adjust: 100%;
    -ms-text-size-adjust: 100%;
    text-size-adjust: 100%;
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    overflow: hidden;
    text-align: center;
}
form {
    background: #fff;
    padding: 40px;
    max-width: 600px;
    margin: 40px auto;
    border-radius: 4px;
    box-shadow: 0 4px 10px 4px rgba(19, 35, 47, 0.3);
}
a{
    float: left;
    width: 50%;

    text-decoration: none;
    color: black;
}
b{
    float: right;
    width: 50%;

    text-decoration: none;
    color: black;
}
input {
    width: 100%;
    padding: 12px 20px;
    margin: 8px 0;
    box-sizing: border-box;
}
    </style>
        <head>
            <title>Create Relationship</title>
        </head>
        <body>
            <br><br>
            <h1>Create Relationship</h1>
            <form action="/createrelation" method="post" enctype="multipart/form-data">
                <label for="database">Database:</label>
                <input type="text" id="database" name="database"><br><br>
                <a>
                <label for="table1">Table 1:</label>
                <input type="text" id="table1" name="table1"><br><br>
                </a>
                <b>
                <label for="table2">Table 2:</label>
                <input type="text" id="table2" name="table2"><br><br>
                </b>
                <a>
                <label for="column1">Column 1:</label>
                <input type="text" id="column1" name="column1"><br><br>
                </a>
                <b>
                <label for="column2">Column 2:</label>
                <input type="text" id="column2" name="column2"><br><br>
                </b>
                <a>
                <label for="ondelete">Delete Action:</label>
                <select id="ondelete" name="ondelete">
                    <option value="cascade">Cascade</option>
                    <option value="restrict">Restrict</option>
                    <option value="setnull">Set Null</option>
                    <option value="setdefault">Set Default</option>
                </select><br><br>
                </a>
                <b>
                <label for="onupdate">Update Action:</label>
                <select id="onupdate" name="onupdate">
                    <option value="cascade">Cascade</option>
                    <option value="restrict">Restrict</option>
                    <option value="setnull">Set Null</option>
                    <option value="setdefault">Set Default</option>
                </select><br><br>
                </b>
                <input type="submit" value="Submit">
            </form>
        </body>
    </html>"#;
    html.to_string()
}
pub fn storerelationform(
    form: CreateRelation,
) -> String{
    for f in form.files {
        println!("file name: {:?}", f.file_name);
        let path = format!("tmp/relation/{}", f.file_name.clone().unwrap());
        println!("path: {:?}", path);
        let newfile=path.clone();
        log::info!("saving to {path}");
        let _=f.file.persist(path);
        return newfile;
    }
    "error".to_string()
    
}




pub fn uploadnewcols(
    form: CreateTable,
) -> String{
    for f in form.files {
        println!("file name: {:?}", f.file_name);
        let path = format!("tmp/data/{}", f.file_name.clone().unwrap());
        println!("path: {:?}", path);
        let newfile=path.clone();
        log::info!("saving to {path}");
        let _=f.file.persist(path);
        return newfile;
    }
    "error".to_string()
    
}
pub fn file_upload(
    form: UploadForm,
) -> String{
    for f in form.files {
        let path = format!("tmp/data/{}", f.file_name.clone().unwrap());
        let newfile=path.clone();
        log::info!("saving to {path}");
        let _=f.file.persist(path);
        return newfile;
    }
    "error".to_string()
    
}



pub fn fileinsert() -> String{
    let html = r#"<html>
        <style>
    body {
    background-color: lightblue;
    font-family: 'Roboto', sans-serif;
    font-weight: 300;
    font-size: 14px;
    color: #666666;
    -webkit-font-smoothing: antialiased;
    -webkit-text-size-adjust: 100%;
    -ms-text-size-adjust: 100%;
    text-size-adjust: 100%;
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    overflow: hidden;
    text-align: center;
}
form {
    background: #fff;
    padding: 40px;
    max-width: 600px;
    margin: 40px auto;
    border-radius: 4px;
    box-shadow: 0 4px 10px 4px rgba(19, 35, 47, 0.3);
} 
input {
    width: 100%;
    padding: 12px 20px;
    margin: 8px 0;
    box-sizing: border-box;
}
</style>
        <head><title>Upload Test</title></head>
        <body>
            <form label="Bulk Upload"  target="/upload" method="post" enctype="multipart/form-data">
                <input type="text" label="Database Name" name="database"/>
                <input type="text" label="Table Name" name="table"/>
                <input type="file" label="File" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
            <br>
            <form action='/main'><input type='submit' value='Return to Main Page'></form>
        </body>
    </html>"#;
    html.to_string()

}

use crate::NewRecord;
pub fn buildform(database:&str, table:&str, columns: Vec<String>)->String{

    let mut html= String::new();
    html.push_str("<html><head><title>Query Results</title>");
    html.push_str("<style>
 body{background-color: linen}</style>");

    html.push_str("</head><body>");
    html.push_str("<h1>");
    html.push_str("Insert Into ");
    html.push_str(database);
    html.push_str(".");
    html.push_str(table);
    html.push_str("</h1>");
    html.push_str("<form action='/create/saveform' method='post'>");
    for i in 0..columns.len(){
        //create struct with each column name

        html.push_str("<label for='");
        html.push_str(&columns[i]);
        html.push_str(": ");
        html.push_str("'>");
        html.push_str(&columns[i]);
        html.push_str("</label>");
        html.push_str("<input type='text' id='");
        html.push_str(&columns[i]);
        html.push_str("' name='data");
        html.push_str("'><br><br>");
    }
    html.push_str("<input type='submit' value='Save'></form>");
    html.push_str("<form action='/main'><input type='submit' value='Return to Main Page'></form></body>");
    html
}

//take user response and generate it back to user
pub fn formresponse(columns: NewRecord)->String{
    let mut html= String::new();
    html.push_str("<html><head><title>Create Results</title>");
    html.push_str("<style>
 body{background-color: linen}</style>");

    html.push_str("</head><body>");
    html.push_str("<h1>");
    html.push_str("Insert Into ");
    html.push_str("</h1>");
    html.push_str("<table>");
    for i in 0..columns.records.len(){
        html.push_str("<tr><td>");
        html.push_str(&columns.records[i]);
        html.push_str("</td></tr>");
    }
   html.push_str("</table>");
   html.push_str("<br><br>");
  

    html.push_str("<form action='/main'><input type='submit' value='Return to Main Page'></form></body>");
    html
}

/// Example of the old manual way of processing multipart forms.
#[allow(unused)]
async fn save_file_manual(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}
