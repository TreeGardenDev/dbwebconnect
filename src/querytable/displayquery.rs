
pub fn buildhtml(querydata: Vec<Vec<String>>)->String{
    let mut html = String::new();
    html.push_str("<html><head><title>Query Results</title></head><body><table>");
    for row in querydata{
        html.push_str("<tr>");
        for col in row{
            html.push_str("<td>");
            html.push_str(&col);
            html.push_str("</td>");
        }
        html.push_str("</tr>");
    }
    html.push_str("</table></body></html>");
    html.push_str("<form action='/'><input type='submit' value='Return to Main Page'></form></body>");
    html
}
