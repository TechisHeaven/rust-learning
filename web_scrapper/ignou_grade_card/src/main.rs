use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    process::exit,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client
    let client = Client::new();
    let program_code = "BCA";

    // ! change starting number of enrollment number here
    let starting_number: i128 = 2251090000;

    // Open a file to write the results
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 0..100000 {
        // Format the enrollment number with leading zeros to make it 10 digits
        let enrollment_number = format!("{:010}", starting_number + i);
        let client_request_string = format!(
            "https://gradecard.ignou.ac.in/gradecard/view_gradecard.aspx?eno={}&prog={}&type=1",
            enrollment_number, program_code
        );

        // Perform the HTTP GET request
        let response = match client
            .get(&client_request_string)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Cache-Control", "max-age=0")
            .header("Connection", "keep-alive")
            .header("Cookie", "ASP.NET_SessionId=tw0tdhjadxlozxfhij4hw4hf")
            .header("Referer", "https://gradecard.ignou.ac.in/gradecard/")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Sec-Fetch-User", "?1")
            .header("Sec-GPC", "1")
            .header("Upgrade-Insecure-Requests", "1")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .send() {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Error fetching URL {}: {}", client_request_string, e);
                    exit(1);
                }
            };

        // Parse the HTML response
        let document = Html::parse_document(&response.text()?);

        // Define the CSS selector for the marks table
        let table_selector = Selector::parse("#ctl00_ContentPlaceHolder1_gvDetail").unwrap();
        let enrolment_selector =
            Selector::parse("#ctl00_ContentPlaceHolder1_lblDispEnrolno").unwrap();
        let name_selector = Selector::parse("#ctl00_ContentPlaceHolder1_lblDispname").unwrap();

        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        // Extract enrollment number
        let enrolment_number = match document.select(&enrolment_selector).next() {
            Some(enrolment_element) => enrolment_element
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string(),
            None => {
                println!("Enrolment number element not found");
                continue;
            }
        };

        println!("Enrolment Number: {}", enrolment_number);

        // Extract name
        let name = match document.select(&name_selector).next() {
            Some(name_element) => name_element
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string(),
            None => {
                println!("Name element not found");
                continue;
            }
        };

        println!("Name: {}", name);

        // Find the marks table
        if let Some(table) = document.select(&table_selector).next() {
            // Iterate over the rows in the table
            for row in table.select(&row_selector).skip(1) {
                // Skip the header row
                let cells: Vec<_> = row.select(&cell_selector).collect();

                if cells.len() > 1 {
                    let subject_code = cells[0]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let assignment_marks = cells[1]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let lab1 = cells[2]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let lab2 = cells[3]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let lab3 = cells[4]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let lab4 = cells[5]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let term_end_marks = cells[6]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let practical_marks = cells[7]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();
                    let status = cells[8]
                        .text()
                        .collect::<Vec<_>>()
                        .concat()
                        .trim()
                        .to_string();

                    // Write data to the file
                    writeln!(
                        &mut writer,
                        "Enrolment Number: {}\nName: {}\nSubject Code: {}\nAssignment Marks: {}\nLab1 Marks: {}\nLab2 Marks: {}\nLab3 Marks: {}\nLab4 Marks: {}\nTerm End Marks: {}\nPractical Marks: {}\nStatus: {}\n--------------------------------",
                        enrolment_number, name, subject_code, assignment_marks, lab1, lab2, lab3, lab4, term_end_marks, practical_marks, status
                    )?;
                }
            }
        } else {
            writeln!(
                &mut writer,
                "################################################"
            )?;
        }
    }

    // Close the file
    writer.flush()?;
    Ok(())
}
