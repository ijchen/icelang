pub fn format_as_node(head: &str, children: Vec<String>) -> String {
    let mut output = String::new();

    // Head
    output.push_str("● ");
    output.push_str(head);

    // Children
    let last_child_index = children.len() as isize - 1;
    for (child_index, child) in children.into_iter().enumerate() {
        for (line_index, line) in child.lines().enumerate() {
            output.push('\n');
            output.push_str(
                match (child_index == last_child_index as usize, line_index == 0) {
                    (true, true) => "└─",
                    (true, false) => "  ",
                    (false, true) => "├─",
                    (false, false) => "│ ",
                },
            );

            output.push_str(line);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_just_head() {
        assert_eq!(format_as_node("Hello, world!", vec![]), "● Hello, world!");
    }

    #[test]
    fn test_single_child() {
        assert_eq!(
            format_as_node("Hello, world!", vec!["hey".to_string()]),
            "\
● Hello, world!
└─hey"
        );
    }

    #[test]
    fn test_single_multiline_child() {
        assert_eq!(
            format_as_node("Hello, world!", vec!["hey\nwhat's up\nhello".to_string()]),
            "\
● Hello, world!
└─hey
  what's up
  hello"
        );
    }

    #[test]
    fn test_two_children() {
        assert_eq!(
            format_as_node("Hello, world!", vec!["hey".to_string(), "one".to_string()]),
            "\
● Hello, world!
├─hey
└─one"
        );
    }

    #[test]
    fn test_two_multiline_children() {
        assert_eq!(
            format_as_node(
                "Hello, world!",
                vec![
                    "hey\nwhat's up\nhello".to_string(),
                    "one\ntwo\nthree".to_string()
                ]
            ),
            "\
● Hello, world!
├─hey
│ what's up
│ hello
└─one
  two
  three"
        );
    }

    #[test]
    fn test_large() {
        assert_eq!(
            format_as_node(
                "This is gonna be a big tree",
                vec![
                    format_as_node(
                        "Hello, world!",
                        vec!["hey\nwhat's up\r\nhello".to_string(), "one\r\ntwo\nthree".to_string()],
                    ),
                    format_as_node("Wowzers", vec![]),
                    format_as_node(
                        "This is a big bunch of words",
                        vec![
                            "delayed, like I'd worry about lag".to_string(),
                            "moves\nlike\njagger!\n\n".to_string(),
                            "when, last night?".to_string(),
                            "ten seconds\r\nago".to_string(),
                        ],
                    ),
                    "Turtle Island".to_string(),
                    format_as_node(
                        "Chess",
                        vec![
                            format_as_node(
                                "1. e4 e5",
                                vec![
                                    format_as_node(
                                        "2. Nf3 Nc6",
                                        vec![
                                            "3. Bb5\nRuy Lopez\nSpanish".to_string(), "3. Bc4\nItalian".to_string()
                                        ],
                                    ),
                                    format_as_node(
                                        "2. Nf3 Nf6",
                                        vec![
                                            "Russian game\n(can lead to Stafford gambit, very fun)\nOh no, my queen!".to_string()
                                        ]
                                    ),
                                ],
                            ),
                            format_as_node(
                                "1. e4 c5",
                                vec!["Sicilian, I don't know any theory here".to_string()]
                            ),
                            format_as_node(
                                "1. d4 d5",
                                vec!["Have you seen The Queen's Gambit on Netflix?".to_string()]
                            ),
                        ]
                    )
                ]
            ),
            "\
● This is gonna be a big tree
├─● Hello, world!
│ ├─hey
│ │ what's up
│ │ hello
│ └─one
│   two
│   three
├─● Wowzers
├─● This is a big bunch of words
│ ├─delayed, like I'd worry about lag
│ ├─moves
│ │ like
│ │ jagger!
│ │ 
│ ├─when, last night?
│ └─ten seconds
│   ago
├─Turtle Island
└─● Chess
  ├─● 1. e4 e5
  │ ├─● 2. Nf3 Nc6
  │ │ ├─3. Bb5
  │ │ │ Ruy Lopez
  │ │ │ Spanish
  │ │ └─3. Bc4
  │ │   Italian
  │ └─● 2. Nf3 Nf6
  │   └─Russian game
  │     (can lead to Stafford gambit, very fun)
  │     Oh no, my queen!
  ├─● 1. e4 c5
  │ └─Sicilian, I don't know any theory here
  └─● 1. d4 d5
    └─Have you seen The Queen's Gambit on Netflix?"
        );
    }
}
