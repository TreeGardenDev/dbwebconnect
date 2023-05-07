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



//use file from Insert form in file_upload2
pub fn file_upload(
    form: UploadForm,
) -> String{
    for f in form.files {
        let path = format!("tmp/{}", f.file_name.clone().unwrap());
        let newfile=path.clone();
        log::info!("saving to {path}");
        let _=f.file.persist(path);
        return newfile;
    }
    "error".to_string()
    
}



pub fn fileinsert() -> String{
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <input type="text" name="database"/>
                <input type="text" name="table"/>
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
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
    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
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
  

    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
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
