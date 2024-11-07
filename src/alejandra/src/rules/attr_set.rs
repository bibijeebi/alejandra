pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();
    let mut children = crate::children::Children::new(build_ctx, node);

    let items_count = node
        .children_with_tokens()
        .skip_while(|element| {
            element.kind() != rnix::SyntaxKind::TOKEN_CURLY_B_OPEN
        })
        .take_while(|element| {
            element.kind() != rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE
        })
        .filter(|element| {
            matches!(
                element.kind(),
                rnix::SyntaxKind::NODE_KEY_VALUE
                    | rnix::SyntaxKind::NODE_INHERIT
                    | rnix::SyntaxKind::NODE_INHERIT_FROM
                    | rnix::SyntaxKind::TOKEN_COMMENT
            )
        })
        .count();

    let vertical = items_count > 1
        || children.has_comments()
        || children.has_newlines()
        || build_ctx.vertical;

    // rec
    let child = children.peek_next().unwrap();
    if let rnix::SyntaxKind::TOKEN_REC = child.kind() {
        steps.push_back(crate::builder::Step::Format(child));
        children.move_next();

        if let rnix::SyntaxKind::TOKEN_COMMENT
        | rnix::SyntaxKind::TOKEN_WHITESPACE =
            children.peek_next().unwrap().kind()
        {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        } else {
            steps.push_back(crate::builder::Step::Whitespace);
        }
    }

    // /**/
    children.drain_trivia(|element| match element {
        crate::children::Trivia::Comment(text) => {
            steps.push_back(crate::builder::Step::Comment(text));
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
        }
        crate::children::Trivia::Whitespace(_) => {}
    });

    // {
    let child = children.get_next().unwrap();
    steps.push_back(crate::builder::Step::Format(child));
    if vertical {
        steps.push_back(crate::builder::Step::Indent);
    }

    // Collect entries
    let mut entries = Vec::new();
    while let Some(child) = children.peek_next() {
        if matches!(
            child.kind(),
            rnix::SyntaxKind::NODE_KEY_VALUE | rnix::SyntaxKind::NODE_INHERIT
        ) {
            entries.push(child.clone());
            children.move_next();
        } else if child.kind() == rnix::SyntaxKind::TOKEN_CURLY_B_CLOSE {
            break;
        } else {
            children.move_next();
        }
    }

    // Sort entries by key name
    let sorted_entries = crate::sort::sort_attr_set_entries(&entries);

    // Format sorted entries
    for entry in sorted_entries {
        if vertical {
            steps.push_back(crate::builder::Step::NewLine);
            steps.push_back(crate::builder::Step::Pad);
            steps.push_back(crate::builder::Step::FormatWider(entry));
        } else {
            if entries.len() > 1 {
                steps.push_back(crate::builder::Step::Whitespace);
            }
            steps.push_back(crate::builder::Step::Format(entry));
        }
    }

    // }
    if vertical {
        steps.push_back(crate::builder::Step::Dedent);
        steps.push_back(crate::builder::Step::NewLine);
        steps.push_back(crate::builder::Step::Pad);
    }
    steps.push_back(crate::builder::Step::Format(children.get_next().unwrap()));

    steps
}
