// Purpose: generate form for table

pub fn buildform(database:&str, table:&str, columns: Vec<String>)->String{
    let mut html= String::new();
    html.push_str("<html><head><title>Query Results</title>");
    html.push_str("<style>
 body{background-color: linen}</style>");

    html.push_str("</head><body>");
    html.push_str("<h1>");
    html.push_str("Insert Into");
    html.push_str(database);
    html.push_str(".");
    html.push_str(table);
    html.push_str("</h1>");
    html.push_str("<table>");
    for col in &columns{
        html.push_str("<th>");
        html.push_str(col);
        html.push_str("</th>");
    }
    html.push_str("</table></body></html>");
    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
    html
}


