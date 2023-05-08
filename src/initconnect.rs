//initialize conenction parameters like username, password, host, port
use crate::LinkDataBase;
use std::fs::File;
use std::io::Write;


pub fn getpagehtml() -> String{
    //get page html to type username, password, host, port.
    let mut html = String::new();

    html.push_str("<html>");
    html.push_str("<head>");
   //add css
    html.push_str("<style>");
    html.push_str("body {
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

}");
    
    html.push_str("<title>Database Connection</title>");
    html.push_str("</head>");
    html.push_str("<body>");
    html.push_str("<form action=\"/\" method=\"post\">");
    html.push_str("<label for=\"dbuser\">Username:</label>");
    html.push_str("<input type=\"text\" id=\"dbuser\" name=\"dbuser\"><br><br>");
    html.push_str("<label for=\"dbpass\">Password:</label>");
    html.push_str("<input type=\"text\" id=\"dbpass\" name=\"dbpass\"><br><br>");
    html.push_str("<label for=\"dbhost\">Host:</label>");
    html.push_str("<input type=\"text\" id=\"dbhost\" name=\"dbhost\"><br><br>");
    html.push_str("<label for=\"dbport\">Port:</label>");
    html.push_str("<input type=\"text\" id=\"dbport\" name=\"dbport\"><br><br>");
    html.push_str("<input type=\"submit\" value=\"Submit\">");
    html.push_str("</form>");
    html.push_str("</body>");
    html.push_str("</html>");
//    println!("{}", html);

    html
}

pub fn postdatabaseconnection(form: LinkDataBase) {
    //get form data from getpagehtml
    //use form data to connect to database
    //return connection
    //let mut connection=dbconnect::database_connection(&form.database.to_string());
    //get user input from form data from create function
    let username = form.dbuser;
    let password = form.dbpass;
    let host = form.dbhost;
    let port = form.dbport;
    //save results to file in tmp directory
    let mut file = File::create("tmp/dbconnection.txt").unwrap();
    file.write_all(username.as_bytes()).unwrap();
   file.write_all(",".as_bytes()).unwrap();
    file.write_all(password.as_bytes()).unwrap();
   file.write_all(",".as_bytes()).unwrap();
    file.write_all(host.as_bytes()).unwrap();
   file.write_all(",".as_bytes()).unwrap();
    file.write_all(port.as_bytes()).unwrap();
    //return connection

}
