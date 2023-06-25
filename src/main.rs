extern crate mdbook;

use std::io;

use mdbook::book::Chapter;
use mdbook::BookItem;
use mdbook::renderer::RenderContext;

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    let mut sum = 0;
    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            let num_words = count_words(ch);
            // println!("{}: {}", ch.name, num_words);
            sum += num_words;
        }
    }

    println!("Total: {}", sum);
}

fn count_words(ch: &Chapter) -> usize {
    words_count::count(ch.content.as_str()).words
}
