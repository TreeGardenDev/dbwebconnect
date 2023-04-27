// Purpose: generate form for table

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
        html.push_str("<label for='");
        html.push_str(&columns[i]);
        html.push_str(": ");
        html.push_str("'>");
        html.push_str(&columns[i]);
        html.push_str("</label>");
        html.push_str("<input type='text' id='");
        html.push_str(&columns[i]);
        html.push_str("' name='");
        html.push_str(&columns[i]);
        html.push_str("'><br><br>");
    }
    html.push_str("<input type='submit' value='Save'></form>");
    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
    html
}

//take user input from form and save to csv file

