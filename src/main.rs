use std::{env, str::FromStr};

pub mod builder;
pub mod markdown;


use chrono;
use std::fs;

fn main()
{
    // set working directory
    let work_dir = "/home/del/projects/rust/delwgrust/";
    _ = env::set_current_dir(work_dir);

    // templates directory
    let templates_dir = "delwg-del.cx/";

    // generated site directory
    let web_dir = "web/";

    // generating header
    let head = builder::file_to_string(&(templates_dir.to_owned() + "template/head.html"))
        .replace("{TITLE}", "Mihails M");

    // generating site menus
    let mut menu = builder::file_to_string(&(templates_dir.to_owned() + "template/menu.html"));
    let mut menu_blog = menu.clone();
    menu = menu.replace("{PATH}", "");
    menu_blog = menu_blog.replace("{PATH}", "../");

    // generate footer
    let mut footer = builder::file_to_string(&(templates_dir.to_owned() + "template/footer.html"));
    let date = chrono::offset::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let year = chrono::offset::Local::now().format("%Y").to_string();
    footer = footer.replace("{GENERATED_DATE}", &date);
    footer = footer.replace("{CURRENT_YEAR}", &year);

    /****** GENERATING PAGES ******/
    // index.html
    let mut index = builder::Page::new();
    index.head = head.clone();
    index.add_style("style.css");
    index.add_style("textStyle.css");
    let links = builder::file_to_string(&(templates_dir.to_owned() + "template/links.html"));
    let cv = markdown::parse_file(&(templates_dir.to_owned() + "cv.md"));
    index.body += &menu;
    index.body += &links;
    index.body += &cv;
    index.body += &footer;
    index.save(&(work_dir.to_owned() + web_dir + "index.html"));

    // RSS feed
    let mut rss = builder::file_to_string(&(templates_dir.to_owned() + "rss.xml"));
    rss = rss.replace("{YEAR}", &year);
    rss = rss.replace(
        "{LAST_BUILD_DATE}",
        &chrono::offset::Local::now()
            .format("%a, %d %b %Y %T GMT")
            .to_string(),
    );

    // find all blog pages and build them
    let mut blog_links: Vec<String> = vec![];
    let mut blog_links_raw: Vec<String> = vec![];
    for file in fs::read_dir(&(templates_dir.to_owned() + "blog")).unwrap()
    {
        let file_name = file.unwrap().path().display().to_string();
        if file_name.ends_with(".md")
        {
            let page_file = file_name.replace(".md", ".html");
            let mut blog_page = builder::Page::new();
            let blog_markdown = builder::file_to_string(&(work_dir.to_owned() + &file_name));

            blog_page.add_style("../style.css");
            blog_page.add_style("../textStyle.css");
            let blog_content = markdown::parse(&blog_markdown);
            blog_page.body += &menu_blog;
            blog_page.body += &blog_content;
            blog_page.body += &footer;
            blog_page.save(
                &(work_dir.to_owned() + &web_dir + "blog/" + &page_file.split("/").last().unwrap()),
            );

            // add link of this blog page to blog page host
            let tag = "<<TAG>>";
            let marker_start = blog_markdown.find(tag).unwrap() + tag.len();
            let marker_end = blog_markdown[marker_start..].find(tag).unwrap() + tag.len();
            let tag_raw = &blog_markdown[marker_start..marker_end];

            let date = tag_raw.split('|').last().unwrap().replace("-", "·");
            let title = tag_raw.split('|').nth(0).unwrap();

            let link = format!(
                "[{}] <a href=\"blog/{}\">{}</a></br>",
                date,
                &page_file.split("/").last().unwrap(),
                title
            );
            blog_links.push(link);
            blog_links_raw.push(format!(
                "{}|{}|{}",
                title,
                tag_raw.split('|').last().unwrap(),
                "https://del.cx/blog/".to_owned() + &page_file.split("/").last().unwrap()
            ));
        }
    }

    // blog.html
    blog_links.sort();
    blog_links.reverse();
    let links_string = blog_links.join("");

    let mut blog = builder::Page::new();
    blog.head = head.clone();
    blog.add_style("style.css");
    blog.add_style("textStyle.css");
    blog.body += &menu;
    blog.body += &markdown::parse_file(&(templates_dir.to_owned() + "blog.md"));
    blog.body = blog.body.replace("{LINKS}", &links_string);
    blog.body += &footer;
    blog.save(&(work_dir.to_owned() + web_dir + "blog.html"));

    // projects.html
    let mut projects = builder::Page::new();
    projects.head = head.clone();
    projects.add_style("style.css");
    projects.add_style("textStyle.css");
    projects.body += &menu;
    projects.body += &markdown::parse_file(&(templates_dir.to_owned() + "projects.md"));
    projects.body += &footer;
    projects.save(&(work_dir.to_owned() + web_dir + "projects.html"));

    // contact.html
    let mut contact = builder::Page::new();
    contact.head = head.clone();
    contact.add_style("style.css");
    contact.add_style("textStyle.css");
    contact.body += &menu;
    contact.body += &links;
    contact.body += &markdown::parse_file(&(templates_dir.to_owned() + "contact.md"));
    contact.body += &footer;
    contact.save(&(work_dir.to_owned() + web_dir + "contact.html"));

    // RSS feed items
    let rss_item_template = builder::file_to_string(&(templates_dir.to_owned() + "rss-item.xml"));
    blog_links_raw.sort();
    blog_links_raw.reverse();
    let mut xml_items = "".to_owned();
    for blog in blog_links_raw
    {
        let title = blog.split('|').nth(0).unwrap();
        let mut date = blog.split('|').nth(1).unwrap().to_owned() + " 22:10:57 +00:00";
        let link = blog.split('|').nth(2).unwrap();
        let dt = chrono::DateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S %z").unwrap();

        let mut rss_item = rss_item_template.clone();
        rss_item = rss_item.replace("{TITLE}", title);
        rss_item = rss_item.replace("{LINK}", link);
        rss_item = rss_item.replace("{DESCRIPTION}", title);
        rss_item = rss_item.replace(
            "{PUB_DATE}",
            &dt.format("%a, %d %b %Y 00:00:00 GMT").to_string(),
        );
        rss = rss.replace(
            "{LAST_PUB_DATE}",
            &dt.format("%a, %d %b %Y 00:00:00 GMT").to_string(),
        );
        xml_items += &format!("{}", rss_item);
    }
    rss = rss.replace("{ITEMS}", &xml_items);
    builder::string_to_file(&rss, &(work_dir.to_owned() + web_dir + "rss.xml"));
}


// Unit tests
include!("test_markdown.rs");
