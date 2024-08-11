use std::str::Chars;

#[derive(Debug)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
}

impl Pattern {
    /// Checks From Given Iter
    pub fn checks(&self, input: &mut Chars) -> bool {
        match input.next() {
            Some(c) => match self {
                Self::Literal(l) => *l == c,
                Self::Digit => c.is_ascii_digit(),
                Self::Alphanumeric => c.is_ascii_alphanumeric(),
                Self::Group(true, postive) => postive.contains(c),
                Self::Group(false, negative) => !negative.contains(c),
            },
            None => false,
        }
    }
}

pub struct Regx<'a> {
    input: &'a str,

    pattern: Vec<Pattern>,
}

impl<'a> Regx<'a> {
    pub fn new(input: &'a str, pattern: &'a str) -> Self {
        let pattern = Regx::build_patterns(pattern);

        Self { input, pattern }
    }

    /// Checks if Given Pattern is present in Input
    pub fn matches(&self) -> bool {
        'outer: for i in 0..self.input.len() {
            let input = &self.input[i..];
            let mut iter = input.chars();

            for pattern in self.pattern.iter() {
                if !pattern.checks(&mut iter) {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }

    /// Build Vec of Patterns
    fn build_patterns(pattern: &'a str) -> Vec<Pattern> {
        let mut pat = Vec::new();
        let mut chars = pattern.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' => match chars.peek() {
                    Some(p) => {
                        match p {
                            'w' => pat.push(Pattern::Alphanumeric),
                            'd' => pat.push(Pattern::Digit),
                            '\\' => pat.push(Pattern::Literal('\\')),
                            err => panic!("Unhandled pattern: {err}"),
                        }
                        chars.next();
                    }
                    None => pat.push(Pattern::Literal('\\')),
                },
                '[' => {
                    if chars.peek().is_none() {
                        pat.push(Pattern::Literal('['));
                        continue;
                    }

                    let mut is_positive = true;
                    let mut group = String::new();

                    if chars.peek().unwrap() == &'^' {
                        is_positive = false;
                        chars.next();
                    }

                    while let Some(&a) = chars.peek() {
                        if a == ']' {
                            chars.next();
                            break;
                        }

                        group.push(a);
                        chars.next();
                    }

                    pat.push(Pattern::Group(is_positive, group));
                }
                l => pat.push(Pattern::Literal(l)),
            }
        }

        pat
    }
}
