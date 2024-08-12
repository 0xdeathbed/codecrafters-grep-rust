#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Start,
    End,
    Group(bool, String),
}

impl Pattern {
    /// Checks From Given Iter
    pub fn checks(&self, input: &mut impl Iterator<Item = char>) -> bool {
        match input.next() {
            Some(c) => match self {
                Self::Literal(l) => *l == c,
                Self::Digit => c.is_ascii_digit(),
                Self::Alphanumeric => c.is_ascii_alphanumeric(),
                Self::Group(true, postive) => postive.contains(c),
                Self::Group(false, negative) => !negative.contains(c),
                remain => unreachable!("{remain:#?} must not be checked"),
            },
            None => false,
        }
    }
}

pub struct Regx<'a> {
    input: &'a str,
    patterns: Vec<Pattern>,
}

impl<'a> Regx<'a> {
    pub fn new(input: &'a str, pattern: &'a str) -> Self {
        let patterns = Regx::build_patterns(pattern);

        Self { input, patterns }
    }

    /// checks if given pattern is present in input
    pub fn matches(&self) -> bool {
        if self.patterns[0] == Pattern::Start {
            self.match_start()
        } else if self.patterns[self.patterns.len() - 1] == Pattern::End {
            self.match_end()
        } else {
            self.match_pattern()
        }
    }

    /// checks if pattern appears anywhere in input
    pub fn match_pattern(&self) -> bool {
        'outer: for i in 0..self.input.len() {
            let mut iter = self.input[i..].chars();

            for pattern in self.patterns.iter() {
                if !pattern.checks(&mut iter) {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }

    /// checks if pattern appears at start
    fn match_start(&self) -> bool {
        let mut iter = self.input[0..].chars();

        for pattern in self.patterns[1..].iter() {
            if !pattern.checks(&mut iter) {
                return false;
            }
        }

        true
    }

    /// checks if pattern appears at end
    fn match_end(&self) -> bool {
        let mut iter = self.input.chars().rev();
        for pattern in self.patterns.iter().rev().skip(1) {
            if !pattern.checks(&mut iter) {
                return false;
            }
        }

        true
    }

    /// Build Vec of Patterns
    fn build_patterns(pattern: &'a str) -> Vec<Pattern> {
        let mut pat = Vec::new();
        let mut chars = pattern.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '^' => pat.push(Pattern::Start),
                '$' => pat.push(Pattern::End),
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
