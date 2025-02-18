use super::*;

pub fn render(out: Output) -> String {
    let mut result = String::new();

    for Line {
        id: line_id,
        indent,
        body,
        comments,
    } in out.lines()
    {
        swrite!(result, if line_id > 0, "\n");

        for comment in comments {
            swrite!(result, for 0..indent, " ");
            swrite!(
                result,
                "{} {}\n",
                &out.fmt.comments_style.separator,
                comment
            );
        }

        swrite!(result, for 0..indent, " ");
        swrite!(result, "{}", body);
    }

    result
}
