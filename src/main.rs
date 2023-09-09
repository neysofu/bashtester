use std::collections::VecDeque;

use clap::Parser;
use markdown::mdast;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let file_contents = std::fs::read_to_string(&args.filename).unwrap();
    let markdown_parse_options = markdown::ParseOptions::gfm();
    let markdown_ast = markdown::to_mdast(&file_contents, &markdown_parse_options).unwrap();
    let code_blocks = get_all_code_blocks(markdown_ast);

    for code_block in code_blocks {
        for line in code_block.value.lines() {
            if line.starts_with("$ ") {
                println!("command to test: {}", &line[2..]);
            }
        }
    }
}

fn get_all_code_blocks(markdown_ast: mdast::Node) -> Vec<mdast::Code> {
    let mut code_blocks = Vec::new();

    let mut nodes: VecDeque<mdast::Node> = markdown_ast
        .children()
        .map(Clone::clone)
        .map(Into::into)
        .unwrap_or_default();
    while let Some(next_node) = nodes.pop_front() {
        if let mdast::Node::Code(code_node) = next_node {
            code_blocks.push(code_node);
        } else {
            let children = next_node.children().map(Vec::as_slice).unwrap_or_default();
            for child in children.iter() {
                nodes.push_front(child.clone());
            }
        }
    }

    code_blocks
}
