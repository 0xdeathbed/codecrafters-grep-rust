use std::{iter::Peekable, mem, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Any,
    OneOrMore(Box<Pattern>),
    ZeroOrOne(Box<Pattern>),
    BackReference(usize),
    Many(Vec<Vec<Pattern>>),
    Group(bool, String),
}

impl Pattern {
    /// Checks From Given Iter
    pub fn checks(
        &self,
        input: &mut Peekable<Chars>,
        backrefrence: &mut Vec<String>,
        backref_idx: &mut usize,
    ) -> (bool, String) {
        let backup = input.clone();
        let mut captured = String::new();

        let status = match input.next() {
            Some(c) => {
                captured.push(c);
                // dbg!(&backrefrence, &input, c);
                let is_present = match self {
                    Self::Any => true,
                    Self::Literal(l) => *l == c,
                    Self::Digit => c.is_ascii_digit(),
                    Self::Alphanumeric => c.is_ascii_alphanumeric(),
                    Self::Group(true, postive) => {
                        postive.contains(c)
                        // *input = backup.clone();
                        // captured.pop();
                        // let mut is_match = false;
                        // for _ in postive.chars() {
                        //     match input.next_if(|&x| postive.contains(x)) {
                        //         Some(_c) => {
                        //             // *input = backup.clone();
                        //             captured.push(_c);
                        //         }
                        //         None => {
                        //             is_match = false;
                        //             break;
                        //         }
                        //     }
                        // }

                        // is_match
                    }
                    Self::Group(false, negative) => {
                        *input = backup.clone();
                        captured.pop();
                        let mut is_match = true;
                        for _ in negative.chars() {
                            match input.next_if(|&x| !negative.contains(x)) {
                                Some(_c) => {
                                    captured.push(_c);
                                }
                                None => {
                                    // *input = backup.clone();
                                    is_match = false;
                                    break;
                                }
                            }
                        }

                        is_match
                    }
                    Self::Many(patterns) => {
                        *input = backup;
                        captured.pop();
                        // *backref_idx += 1;

                        'outer: for pattern in patterns.iter() {
                            let backup_iter = input.clone();
                            for p in pattern.iter() {
                                if let Pattern::BackReference(index) = p {
                                    let captured_pattern =
                                        Regx::build_patterns(&backrefrence[index - 1]);
                                    // dbg!(index, &backrefrence);
                                    for p in captured_pattern {
                                        let result = p.checks(input, backrefrence, backref_idx);
                                        if !result.0 {
                                            *input = backup_iter;
                                            continue 'outer;
                                        }
                                        captured.push_str(&result.1);
                                    }
                                    continue;
                                }

                                let current_backref = *backref_idx;
                                let result = p.checks(input, backrefrence, backref_idx);

                                if !result.0 {
                                    *input = backup_iter;
                                    captured = String::new();
                                    continue 'outer;
                                }

                                captured.push_str(&result.1);

                                if let Pattern::Many(_) = p {
                                    // dbg!(&current_backref, &backref_idx);
                                    backrefrence[current_backref] = result.1;
                                    *backref_idx += 1;
                                }
                            }

                            return (true, captured);
                        }

                        false
                    }
                    Self::ZeroOrOne(p) => {
                        *input = backup.clone();
                        captured.pop();

                        let result = p.checks(input, backrefrence, backref_idx);
                        if !result.0 {
                            *input = backup;
                        }

                        captured.push_str(&result.1);

                        true
                    }
                    Self::OneOrMore(p) => {
                        *input = backup;
                        captured.pop();

                        let mut backup_iter = input.clone();
                        let mut result = p.checks(input, backrefrence, backref_idx);
                        if !result.0 {
                            *input = backup_iter;
                            false
                        } else {
                            captured.push_str(&result.1);
                            // dbg!(&captured);
                            loop {
                                backup_iter = input.clone();
                                if let Pattern::Group(false, _) = **p {
                                    break;
                                }

                                result = p.checks(input, backrefrence, backref_idx);
                                if !result.0 {
                                    *input = backup_iter;
                                    break;
                                }

                                captured.push_str(&result.1);
                            }

                            true
                        }
                    }
                    remain => unreachable!("{remain:#?} must not be checked"),
                };

                if !is_present {
                    captured = String::new();
                }

                is_present
            }
            None => false,
        };

        (status, captured)
    }
}

pub struct Regx<'a> {
    input: &'a str,
    patterns: Vec<Pattern>,
    start_anchor: bool,
    end_anchor: bool,
    backrefernce: Vec<String>,
}

impl<'a> Regx<'a> {
    pub fn new(input: &'a str, mut pattern: &'a str) -> Self {
        let start_anchor = if pattern.starts_with('^') {
            pattern = &pattern[1..];
            true
        } else {
            false
        };
        let end_anchor = if pattern.ends_with('$') {
            pattern = &pattern[..pattern.len() - 1];
            true
        } else {
            false
        };

        let patterns = Regx::build_patterns(pattern);
        let mut backrefernce = Vec::new();
        backrefernce.resize_with(10, Default::default);

        Self {
            start_anchor,
            end_anchor,
            input,
            patterns,
            backrefernce,
        }
    }

    /// checks if given pattern is present in input
    pub fn matches(&mut self) -> bool {
        self.match_pattern()
    }

    /// checks if pattern appears anywhere in input
    pub fn match_pattern(&mut self) -> bool {
        // dbg!(&self.patterns);
        let mut backref_idx;
        'outer: for i in 0..self.input.len() {
            backref_idx = 0;
            let input = &self.input[i..];
            let mut iter = input.chars().peekable();

            for pattern in self.patterns.iter() {
                match pattern {
                    Pattern::BackReference(index) => {
                        // build pattern on captured pattern
                        let captured_pattern = Regx::build_patterns(&self.backrefernce[index - 1]);
                        // dbg!(index, &captured_pattern);
                        for p in captured_pattern {
                            if !p
                                .checks(&mut iter, &mut self.backrefernce, &mut backref_idx)
                                .0
                            {
                                // dbg!(&p);
                                if self.start_anchor {
                                    return false;
                                }

                                continue 'outer;
                            }
                        }
                    }
                    _ => {
                        let current_backref = backref_idx;
                        if let Pattern::Many(_) = pattern {
                            backref_idx += 1;
                        }

                        let result =
                            pattern.checks(&mut iter, &mut self.backrefernce, &mut backref_idx);
                        if !result.0 {
                            if self.start_anchor {
                                return false;
                            }

                            continue 'outer;
                        }

                        // if pattern was in parenthesis store it for future reference
                        if let Pattern::Many(_a) = pattern {
                            self.backrefernce[current_backref] = result.1;
                            // dbg!(&current_backref, backref_idx);
                            // dbg!(&self.backrefernce);
                            // backref_idx += 1;
                        }
                    }
                }
            }

            // All pattern matched
            if self.end_anchor {
                // if at end
                return iter.next().is_none();
            } else {
                return true;
            };
        }

        false
    }

    /// Build Vec of Patterns
    fn build_patterns(pattern: &'a str) -> Vec<Pattern> {
        let mut pat = Vec::new();
        let mut chars = pattern.chars().peekable();
        let mut many_patterns: Vec<Vec<Pattern>> = Vec::new();
        let mut temp: Vec<Pattern> = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    many_patterns = Vec::new();
                    temp = mem::take(&mut pat);
                }
                '|' => {
                    many_patterns.push(pat);
                    pat = Vec::new();
                    // pat = mem::take(&mut temp);
                }
                ')' => {
                    many_patterns.push(pat.clone());
                    pat = mem::take(&mut temp);

                    pat.push(Pattern::Many(mem::take(&mut many_patterns)));
                }

                '.' => pat.push(Pattern::Any),
                '\\' => match chars.peek() {
                    Some(p) => {
                        match p {
                            'w' => pat.push(Pattern::Alphanumeric),
                            'd' => pat.push(Pattern::Digit),
                            '\\' => pat.push(Pattern::Literal('\\')),
                            '(' => pat.push(Pattern::Literal('(')),
                            number if number.is_ascii_digit() => pat.push(Pattern::BackReference(
                                number.to_digit(10).unwrap() as usize,
                            )),
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
                '+' => {
                    if let Some(p) = pat.pop() {
                        pat.push(Pattern::OneOrMore(Box::new(p)));
                    } else {
                        pat.push(Pattern::Literal('+'));
                    }
                }
                '?' => {
                    if let Some(p) = pat.pop() {
                        pat.push(Pattern::ZeroOrOne(Box::new(p)));
                    } else {
                        pat.push(Pattern::Literal('?'));
                    }
                }
                l => pat.push(Pattern::Literal(l)),
            }
        }

        pat
    }
}
