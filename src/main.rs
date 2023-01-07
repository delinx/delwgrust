use std::fs::File;
use std::io::Read;

mod markdown
{
    pub fn parse(source: &str) -> String
    {
        let mut result: String = String::new();
        let mut tmp: String = String::from(source);

        if source.len() < 1
        {
            return result;
        }

        tmp = tmp.replace("\\n", "</br>");
        // divider
        tmp = tmp.replace("\\---", "<hr>");
        // image with style
        tmp = regex::Regex::new(r"!\[(.+?)\]\((.+?)\)\{(.+?)\}")
            .unwrap()
            .replace_all(&tmp, "<img src=\"$2\" alt=\"$1\" style=\"$3\">")
            .to_string();
        // images
        tmp = regex::Regex::new(r"!\[(.+?)\]\((.+?)\)")
            .unwrap()
            .replace_all(&tmp, "<img src=\"$2\" alt=\"$1\">")
            .to_string();
        // links
        tmp = regex::Regex::new(r"\[(.+?)\]\((.+?)\)")
            .unwrap()
            .replace_all(&tmp, "<a href=\"$2\">$1</a>")
            .to_string();
        // bold
        tmp = regex::Regex::new(r"\*\*(.+?)\*\*")
            .unwrap()
            .replace_all(&tmp, "<b>$1</b>")
            .to_string();

        let mut in_paragraph: bool = false;

        for line in tmp.split("\n")
        {
            // TODO: We never open and close p on same line, so might as well make them exclusive
            let mut result_line: String = line.to_string();

            if line.starts_with("<<TAG>>")
            {
                continue;
            }

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
                        else
                        {
                            break;
                        }
                    }
                    count
                };
                result_line = format!(
                    "<h{}>{}</h{}>",
                    header_count,
                    &line[header_count..],
                    header_count
                );
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
    // let mut file = File::open("cv.md").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("{}", markdown::parse(&contents));
}


// Unit tests
include!("test_markdown.rs");
