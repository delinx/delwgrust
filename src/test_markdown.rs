#[test]
fn test_markdown_paragraph()
{
    assert_eq!(
        markdown::parse("hello this is a test."),
        "<p>hello this is a test.</p>"
    );
}

#[test]
fn test_markdown_two_paragraphs()
{
    assert_eq!(
        markdown::parse("hello this is a test.\n\nhello this is a test."),
        "<p>hello this is a test.</p><p>hello this is a test.</p>"
    );
}

#[test]
fn test_markdown_paragraph_with_br()
{
    assert_eq!(
        markdown::parse("hello this is a test.\\nhello this is a test."),
        "<p>hello this is a test.</br>hello this is a test.</p>"
    );
}

#[test]
fn test_markdown_two_paragraph_with_br()
{
    assert_eq!(
        markdown::parse("hello this is a test.\\nhello \nthis\n\n is a \\ntest."),
        "<p>hello this is a test.</br>hello this</p><p> is a </br>test.</p>"
    );
}

#[test]
fn test_markdown_br()
{
    assert_eq!(markdown::parse("Hello\\nWolrd"), "<p>Hello</br>Wolrd</p>");
}

#[test]
fn test_markdown_link()
{
    assert_eq!(
        markdown::parse("[Hello this is a test.](https://delwg.com)"),
        "<p><a href=\"https://delwg.com\">Hello this is a test.</a></p>"
    );
}

#[test]
fn test_markdown_link_inside_text()
{
    assert_eq!(
        markdown::parse("Hello [link](https://delwg.com)\\n this is \n\n a link"),
        "<p>Hello <a href=\"https://delwg.com\">link</a></br> this is </p><p> a link</p>"
    );
}

// test for embedded image
#[test]
fn test_markdown_image()
{
    assert_eq!(
        markdown::parse("![Hello this is a test.](https://delwg.com)"),
        "<p><img src=\"https://delwg.com\" alt=\"Hello this is a test.\"></p>"
    );
}

// test for embedded image with style
#[test]
fn test_markdown_image_with_style()
{
    assert_eq!(
        markdown::parse("![Hello this is a test.](https://delwg.com){width: 100px;}"),
        "<p><img src=\"https://delwg.com\" alt=\"Hello this is a test.\" style=\"width: 100px;\"></p>"
    );
}

// test for ignoring lines starting with <<TAG>>
#[test]
fn test_markdown_ignore_tag()
{
    assert_eq!(markdown::parse("<<TAG>>Hello this is a test.<<TAG>>"), "");
}

// test for divider \---
#[test]
fn test_markdown_divider()
{
    assert_eq!(markdown::parse("\\---"), "<p><hr></p>");
}

// test header 1
#[test]
fn test_markdown_header_1()
{
    assert_eq!(
        markdown::parse("# Hello this is a test."),
        "<p><h1> Hello this is a test.</h1></p>"
    );
}

// test header 4
#[test]
fn test_markdown_header_4()
{
    assert_eq!(
        markdown::parse("#### Hello this is a test."),
        "<p><h4> Hello this is a test.</h4></p>"
    );
}


// test bold text
#[test]
fn test_markdown_bold()
{
    assert_eq!(
        markdown::parse("**Hello this is a test.**"),
        "<p><b>Hello this is a test.</b></p>"
    );
}

// more advanced bold text
#[test]
fn test_markdown_bold_advanced()
{
    assert_eq!(
        markdown::parse("**Hello this is a test.**\\nHello this is a test."),
        "<p><b>Hello this is a test.</b></br>Hello this is a test.</p>"
    );
}
