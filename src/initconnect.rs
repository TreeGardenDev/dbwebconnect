//initialize conenction parameters like username, password, host, port
//use crate::LinkDataBase;
//use std::fs::File;
//use std::io::Write;

pub fn getpagehtml() -> String {
    //get page html to type username, password, host, port.
    let mut html = String::new();

    html.push_str("<html>");
    html.push_str("<head>");
    //add css
    html.push_str("<style>");
    html.push_str(
        "body {
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
    </style>",
    );

    html.push_str("<title>Database Web Connector</title>");
    html.push_str("</head>");
    html.push_str("</html>");
    println!("{}", html);

    html
}
