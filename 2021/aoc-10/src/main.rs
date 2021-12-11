use std::io::{stdin, BufRead};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone,Debug)]
struct Chunk {
    start_tag: char,
    end_tag: char,
    closed: bool,
    chunks: Vec<Chunk>
}

struct ParserState<'a> {
    input: Rc<RefCell<dyn Iterator<Item=char>  + 'a>>,
    current_token: Option<char>,
    chunk: Chunk,
    pos: usize,
}

impl ParserState<'_> {
    fn get_token(&mut self) -> Option<char> {
        if let Some(token) = self.current_token {
            self.current_token = None;
            return Some(token);
        }
        let token = self.input.borrow_mut().next();
        self.pos += 1;
        token
    }

    fn parse(&mut self)  {
        loop {
            if let Some(token) = self.get_token() {
                if token == self.chunk.end_tag {
                    self.chunk.closed = true;
                    return
                }
                let maybe_end_tag = get_end_tag(token);
                if let Some(end_tag) = maybe_end_tag {
                    let mut substate = ParserState{
                        input: self.input.clone(),
                        current_token: None,
                        chunk: Chunk{
                            start_tag: token,
                            end_tag: end_tag,
                            chunks: Vec::new(),
                            closed: false
                        },
                        pos: self.pos
                    };
                    substate.parse();
                    self.pos = substate.pos;
                    if let Some(_unexpected_token) = substate.current_token {
                        self.current_token = substate.current_token;
                        return;
                    }
                    self.chunk.chunks.push(substate.chunk);
                } else {
                    self.current_token = Some(token);
                    return;
                }
            }
            else
            {
                return;
            }
        }
    }
}

fn get_end_tag(start_tag: char) -> Option<char> {
    match start_tag {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None
    }
}

fn tokenize(input :&str) -> Vec<char>{
    input.chars().collect()
}

fn parse(input: &[char]) -> ParserState {
    let mut state = ParserState{
        input: Rc::new(RefCell::new(input.iter().map(|c| *c))),
        current_token: None,
        chunk: Chunk{
            start_tag: ' ',
            end_tag: ' ',
            chunks: Vec::new(),
            closed: false
        },
        pos: 0
    };
    state.parse();

    state
}

fn token_to_score(token: char) -> usize {
    match token {        
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn token_to_ac_points(token: char) -> usize {
    match token {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn ac_score(tokens: &[char]) -> usize {
    tokens.into_iter()
        .fold(0, |s, t| {
            s * 5 + token_to_ac_points(*t)
        })
}

fn get_closing_tags(chunk: &Chunk) -> Vec<char> {
    let mut closing_tags: Vec<char> = chunk.chunks.iter().flat_map(|c| get_closing_tags(&c).into_iter()).collect::<Vec<char>>();
    if !chunk.closed && chunk.end_tag != ' ' {
        closing_tags.push(chunk.end_tag);
    }
    closing_tags
}

fn main() {
    let line_tokens: Vec<Vec<char>> = stdin().lock().lines()
        .map(|l| tokenize(&l.unwrap()))
        .collect();

    let mut score = 0;
    let mut ac_scores: Vec<usize> = Vec::new();
    for line in line_tokens.iter() {
        let state = parse(line);
        if let Some(unexpected_token) = state.current_token {
            println!("Unexpected token: {} at pos {}, chunk: {:?}", unexpected_token, state.pos, state.chunk);
            score += token_to_score(unexpected_token);
        }
        else if !state.chunk.closed {
            let closing_tags = get_closing_tags(&state.chunk);
            let ac_score = ac_score(&closing_tags);
            println!("Unclosed tag, autocomplete: {:?} ac score: {}", closing_tags.into_iter().collect::<String>(), ac_score);
            ac_scores.push(ac_score);
        } 
    }

    ac_scores.sort();
    println!("Syntax error score: {}", score);
    println!("Ac scores: {:?}, middle {}", ac_scores, ac_scores[ac_scores.len()/2])
}
