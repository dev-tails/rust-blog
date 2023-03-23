pub fn markdown_to_html(markdown: String) -> String {
    let mut tag = String::from("p");
    let mut result = String::new();
    let mut line: String = String::new();

    let mut iterator = markdown.chars();

    loop {
        let char_option = iterator.next();
        if char_option.is_none() {
            break;
        }

        let mut c = char_option.unwrap();

        if line.len() == 0 && c == '#' {
            let mut hashtag_count = 1;

            loop {
                if iterator.next() == Some('#') {
                    hashtag_count += 1;
                } else {
                    break;
                }
            }
            tag = format!("h{hashtag_count}");
        } else if c == '[' {
            let mut text = String::new();
            loop {
                c = iterator.next().unwrap();
                if c == ']' {
                    iterator.next();
                    break;
                } else {
                    text.push(c);
                }
            }

            let mut href = String::new();
            loop {
                c = iterator.next().unwrap();
                if c == ')' {
                    break;
                } else {
                    href.push(c);
                }
            }

            line = format!("{line}<a href=\"{href}\">{text}</a>");
        } else if c == '\n' {
            if line.len() != 0 {
                result = format!("{result}<{tag}>{line}</{tag}>");
                line = String::new();
                tag = String::from("p");
            }
        } else {
            line.push(c);
        }
    }

    if line.len() != 0 {
        result = format!("{result}<{tag}>{line}</{tag}>");
    }

    return format!("{result}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_wraps_text_in_p_tag() {
        let result = markdown_to_html(String::from("hello"));
        assert_eq!(result, String::from("<p>hello</p>"));
    }

    #[test]
    fn it_converts_hashtag_to_header() {
        let result = markdown_to_html(String::from("# Hello"));
        assert_eq!(result, String::from("<h1>Hello</h1>"));
    }

    #[test]
    fn it_should_convert_h1_and_p_multiline() {
        let result = markdown_to_html(String::from("# Hello\n\nhello"));
        assert_eq!(result, String::from("<h1>Hello</h1><p>hello</p>"));
    }

    #[test]
    fn it_should_not_convert_hashtag_mid_text() {
        let result = markdown_to_html(String::from("Hello #"));
        assert_eq!(result, String::from("<p>Hello #</p>"));
    }

    #[test]
    fn it_should_convert_h2() {
        let result = markdown_to_html(String::from("## Hello"));
        assert_eq!(result, String::from("<h2>Hello</h2>"));
    }

    #[test]
    fn it_should_convert_h3() {
        let result = markdown_to_html(String::from("### Hello"));
        assert_eq!(result, String::from("<h3>Hello</h3>"));
    }

    #[test]
    fn it_should_convert_anchor() {
        let result = markdown_to_html(String::from("[Text](https://devtails.xyz)"));
        assert_eq!(result, String::from("<p><a href=\"https://devtails.xyz\">Text</a></p>"));
    }

    #[test]
    fn it_should_convert_inline_anchor() {
        let result = markdown_to_html(String::from("Inline link: [Text](https://devtails.xyz)"));
        assert_eq!(result, String::from("<p>Inline link: <a href=\"https://devtails.xyz\">Text</a></p>"));
    }
}
