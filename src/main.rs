// include!("math/Matrix.rs");

mod markdown
{
    pub fn parse(source: String) -> String
    {
        let mut result: String = String::new();

        if source.len() < 1
        {
            return result;
        }

        let mut in_paragraph: bool = false;

        for line in source.split("\n")
        {
            // TODO: We never open and close p on same line, so might as well make them exclusive
            let mut result_line: String = line.to_string();

            // TODO: handle \--- devider
            // TODO: handle <<TAG>><<TAG>>

            result_line = line.replace("\\n", "</br>");

            // if not in paragragh then make one
            if !in_paragraph
            {
                result.push_str("<p>");
                in_paragraph = true;
            }

            // parse empty line cases
            if line == ""
            {
                // end the paragraph
                if in_paragraph
                {
                    result.push_str("</p>");
                    in_paragraph = false;
                }
                else
                {
                    // FIXME: Not sure if this case will ever trigger
                    println!("INFO: Backup new line was triggered");
                    result.push_str("</br>");
                }
                continue;
            }

            // 1st char
            let first_char = line.chars().nth(0).unwrap();
            // handle titles
            if first_char == '#'
            {
                let header_count = {
                    let mut count = 0;
                    for chars in line.chars()
                    {
                        if chars == '#'
                        {
                            count += 1;
                        }
                    }
                    count
                };
                println!("INFO: Header {}", header_count);
            }


            result.push_str(&result_line);
        }

        // close paragraph if still open
        if in_paragraph
        {
            result.push_str("</p>");
        }

        return result;
    }
}

fn main()
{
    println!("Hello, world!");
    println!(
        "{}",
        markdown::parse(format!(
            "hello world\\nnew line\n\nand another one\n#test\n##test\n\ntest"
        ))
    );
}
