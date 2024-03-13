fn main() {
    let context_lines = 2;
    let search_term = "oo";
    let quote = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (index, line) in quote.lines().enumerate(){
        if line.contains(search_term) {
            tags.push(index);
        }

        let v = Vec::with_capacity(2*context_lines + 1);
        ctx.push(v);
    }

    if tags.len() == 0 {
        return;
    }
    
    for (index, line) in quote.lines().enumerate(){
        for(j, tag) in tags.iter().enumerate(){
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if(index >= lower_bound) && (index <= upper_bound){
                let line_as_string = String::from(line);
                let local_ctx = (index, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter(){
        for &(index, ref line) in local_ctx.iter(){
            let line_num = index + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

