
pub fn buildhtml(querydata: Vec<Vec<String>>, database:&str, table:&str, columns: Vec<String>)->String{
    let mut html = String::new();
    html.push_str("<html><head><title>Query Results</title>");
    html.push_str("<style>table, tr { display: block; float: left; }
th, td { display: block; } body{background-color: linen}</style>");

    html.push_str("</head><body>");
    html.push_str("<h1>");
    html.push_str("Query Results for ");
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
    for row in querydata{
        html.push_str("<tr>");
        for col in &row{
            html.push_str("<td>");
            html.push_str(&col);
           //transpose table 
                 
            html.push_str("</td>");

        }
        
        html.push_str("</tr>");
    }
    html.push_str("</table></body></html>");
    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
    html
}
