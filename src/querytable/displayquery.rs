
pub fn buildhtml(querydata: Vec<Vec<String>>, database:&str, table:&str, columns: Vec<String>)->String{
    let mut html = String::new();
    html.push_str("<html><head><title>Query Results</title>");
    html.push_str("<style>
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
    </style>");

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
    html.push_str("<form action='/main'><input type='submit' value='Return to Main Page'></form></body>");
    html
}
