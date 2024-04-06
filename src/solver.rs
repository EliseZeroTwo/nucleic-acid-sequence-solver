use std::ops::Index;

use crate::error::{FResult, FactoringError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    A,
    C,
    G,
    U,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PossibilityTable {
    pub a: bool,
    pub c: bool,
    pub g: bool,
    pub u: bool,
}

impl<'t> IntoIterator for &'t PossibilityTable {
    type Item = Base;

    type IntoIter = PossibilityTableIter<'t>;

    fn into_iter(self) -> Self::IntoIter {
        PossibilityTableIter {
            next: Some(Base::A),
            table: self,
        }
    }
}

pub struct PossibilityTableIter<'t> {
    pub next: Option<Base>,
    pub table: &'t PossibilityTable,
}

impl<'t> Iterator for PossibilityTableIter<'t> {
    type Item = Base;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next {
                Some(Base::A) => {
                    self.next = Some(Base::C);
                    if self.table.a {
                        return Some(Base::A);
                    }
                }
                Some(Base::C) => {
                    self.next = Some(Base::G);
                    if self.table.c {
                        return Some(Base::C);
                    }
                }
                Some(Base::G) => {
                    self.next = Some(Base::U);
                    if self.table.g {
                        return Some(Base::G);
                    }
                }
                Some(Base::U) => {
                    self.next = None;
                    if self.table.u {
                        return Some(Base::U);
                    }
                }
                None => return None,
            }
        }
    }
}

impl Index<Base> for PossibilityTable {
    type Output = bool;

    fn index(&self, index: Base) -> &Self::Output {
        match index {
            Base::A => &self.a,
            Base::C => &self.c,
            Base::G => &self.g,
            Base::U => &self.u,
        }
    }
}

pub struct Possibilities {
    pub a: PossibilityTable,
    pub c: PossibilityTable,
    pub g: PossibilityTable,
    pub u: PossibilityTable,

    pub w: PossibilityTable,
    pub s: PossibilityTable,
    pub m: PossibilityTable,
    pub k: PossibilityTable,
    pub r: PossibilityTable,
    pub y: PossibilityTable,

    pub b: PossibilityTable,
    pub d: PossibilityTable,
    pub h: PossibilityTable,
    pub v: PossibilityTable,

    pub n: PossibilityTable,
}

impl Index<AmbiguousBases> for Possibilities {
    type Output = PossibilityTable;

    fn index(&self, index: AmbiguousBases) -> &Self::Output {
        match index {
            AmbiguousBases::A => &self.a,
            AmbiguousBases::C => &self.c,
            AmbiguousBases::G => &self.g,
            AmbiguousBases::U => &self.u,
            AmbiguousBases::W => &self.w,
            AmbiguousBases::S => &self.s,
            AmbiguousBases::M => &self.m,
            AmbiguousBases::K => &self.k,
            AmbiguousBases::R => &self.r,
            AmbiguousBases::Y => &self.y,
            AmbiguousBases::B => &self.b,
            AmbiguousBases::D => &self.d,
            AmbiguousBases::H => &self.h,
            AmbiguousBases::V => &self.v,
            AmbiguousBases::N => &self.n,
        }
    }
}

pub const POSSIBILITIES: Possibilities = Possibilities {
    a: PossibilityTable {
        a: true,
        c: false,
        g: false,
        u: false,
    },
    c: PossibilityTable {
        a: false,
        c: true,
        g: false,
        u: false,
    },
    g: PossibilityTable {
        a: false,
        c: false,
        g: true,
        u: false,
    },
    u: PossibilityTable {
        a: false,
        c: false,
        g: false,
        u: true,
    },
    w: PossibilityTable {
        a: true,
        c: false,
        g: false,
        u: true,
    },
    s: PossibilityTable {
        a: false,
        c: true,
        g: true,
        u: false,
    },
    m: PossibilityTable {
        a: true,
        c: true,
        g: false,
        u: false,
    },
    k: PossibilityTable {
        a: false,
        c: false,
        g: true,
        u: true,
    },
    r: PossibilityTable {
        a: true,
        c: false,
        g: true,
        u: false,
    },
    y: PossibilityTable {
        a: false,
        c: true,
        g: false,
        u: true,
    },
    b: PossibilityTable {
        a: false,
        c: true,
        g: true,
        u: true,
    },
    d: PossibilityTable {
        a: true,
        c: false,
        g: true,
        u: true,
    },
    h: PossibilityTable {
        a: true,
        c: true,
        g: false,
        u: true,
    },
    v: PossibilityTable {
        a: true,
        c: true,
        g: true,
        u: false,
    },
    n: PossibilityTable {
        a: true,
        c: true,
        g: true,
        u: true,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguousBases {
    /// A
    A,
    /// C
    C,
    /// G
    G,
    /// U
    U,

    /// AT
    W,
    /// CG
    S,
    /// AC
    M,
    /// GT
    K,
    /// AG
    R,
    /// CT
    Y,

    /// CGT: not A (B comes after A)
    B,
    /// AGT: not C (D comes after C)
    D,
    /// ACT: not G (H comes after G)
    H,
    /// ACG: not T (V comes after T and U)
    V,

    /// ACGT
    N,
}

impl AmbiguousBases {
    pub fn from_char(c: char) -> FResult<Self> {
        match c {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' | 'U' => Ok(Self::U),
            'W' => Ok(Self::W),
            'S' => Ok(Self::S),
            'M' => Ok(Self::M),
            'K' => Ok(Self::K),
            'R' => Ok(Self::R),
            'Y' => Ok(Self::Y),
            'B' => Ok(Self::B),
            'D' => Ok(Self::D),
            'H' => Ok(Self::H),
            'V' => Ok(Self::V),
            'N' => Ok(Self::N),
            c => Err(FactoringError::LetterNotAmbiguousBase(c)),
        }
    }
}

macro_rules! finalise {
    ($f:ident, $s:ident, $t:ident, {
        $($($fc:ident:$sc:ident:$tc:ident)|+ = $l:literal),*
    }, ($($fu:ident:$su:ident:$tu:ident),*)) => {
        match ($f, $s, $t) {
            $(
                $((Base::$fc, Base::$sc, Base::$tc))|+ => Some($l),
            )*
            $((Base::$fu, Base::$su, Base::$tu))|* => None,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chain {
    pub first: Base,
    pub second: Option<Base>,
    pub third: Option<Base>,
}

impl Chain {
    pub fn new(first: Base) -> Self {
        Self {
            first,
            second: None,
            third: None,
        }
    }

    pub fn insert(&mut self, next: Base) -> FResult<()> {
        if self.second.is_none() {
            self.second = Some(next);
            Ok(())
        } else if self.third.is_none() {
            self.third = Some(next);
            Ok(())
        } else {
            Err(FactoringError::AttemptedInsertionOnFullChain(*self, next))
        }
    }

    pub fn finalise(self) -> FResult<Option<char>> {
        let first = self.first;
        let second = self
            .second
            .ok_or(FactoringError::FinaliseCalledOnIncompleteChain(self))?;
        let third = self
            .third
            .ok_or(FactoringError::FinaliseCalledOnIncompleteChain(self))?;

        Ok(finalise!(first, second, third, {
            A:G:G | A:G:A = 'R',
            A:G:C | A:G:U = 'S',
            A:A:G | A:A:A = 'K',
            A:A:C | A:A:U = 'N',
            A:C:G | A:C:A | A:C:C | A:C:U = 'T',
            A:U:G = 'M',
            A:U:A | A:U:C | A:U:U = 'I',
            C:G:G | C:G:A | C:G:C | C:G:U = 'R',
            C:A:G | C:A:A = 'Q',
            C:A:C | C:A:U ='H',
            C:C:G | C:C:A | C:C:C | C:C:U = 'P',
            C:U:G | C:U:A | C:U:C | C:U:U = 'L',
            U:G:G = 'W',
            U:G:C | U:G:U = 'C',
            U:A:C | U:A:U = 'Y',
            U:C:G | U:C:A | U:C:C | U:C:U = 'S',
            U:U:G | U:U:A = 'L',
            U:U:C | U:U:U = 'F',
            G:G:G | G:G:A | G:G:C | G:G:U = 'G',
            G:A:G | G:A:A = 'E',
            G:A:C | G:A:U = 'D',
            G:C:G | G:C:A | G:C:C | G:C:U = 'A',
            G:U:G | G:U:A | G:U:C | G:U:U = 'V'
        }, (U:G:A, U:A:G, U:A:A)))
    }
}

pub fn possible_chains(first: char, second: char, third: char) -> FResult<Vec<Chain>> {
    let mut acids: Vec<Chain> = Vec::new();

    let base = AmbiguousBases::from_char(first)?;
    let possibilities = &POSSIBILITIES[base];

    for base in possibilities {
        acids.push(Chain::new(base));
    }

    for letter in [second, third] {
        let base = AmbiguousBases::from_char(letter)?;
        let possibilities = &POSSIBILITIES[base];

        let mut iter_bases: Vec<Chain> = Vec::new();

        for base in possibilities {
            for mut acid in acids.clone() {
                acid.insert(base)?;
                iter_bases.push(acid);
            }
        }

        if !iter_bases.is_empty() {
            acids = iter_bases;
        }
    }

    Ok(acids)
}

pub fn possibilities(input: &str) -> FResult<Vec<String>> {
    let padded_start = input.starts_with('5');
    let mut chars = input.chars();

    if padded_start {
        chars.next();
    }

    let mut possibilities = Vec::<Vec<String>>::new();
    let mut strings = Vec::<String>::new();

    let mut looking_for_start_code = true;

    let mut finished = false;

    'outer: while !finished {
        let Some(next) = chars.next() else {
            break;
        };

        if next == '3' {
            break;
        }

        if looking_for_start_code {
            let mut forked = chars.clone();

            let second = match forked.next() {
                Some('3') => {
                    #[cfg(debug_assertions)]
                    println!("second was 3 when searching");
                    finished = true;
                    'N'
                }
                Some(second) => second,
                None => {
                    #[cfg(debug_assertions)]
                    println!("second was missing when searching");
                    finished = true;
                    'N'
                }
            };

            let third = match forked.next() {
                Some('3') => {
                    #[cfg(debug_assertions)]
                    println!("third was 3 when searching");
                    finished = true;
                    'N'
                }
                Some(third) => third,
                None => {
                    #[cfg(debug_assertions)]
                    println!("third was missing when searching");
                    finished = true;
                    'N'
                }
            };

            let chains = possible_chains(next, second, third)?;

            if chains.is_empty() {
                continue 'outer;
            }

            for chain in chains {
                if !chain.finalise()?.map(|x| x == 'M').unwrap_or_default() {
                    continue 'outer;
                }
            }

            looking_for_start_code = false;
            #[cfg(debug_assertions)]
            println!("Hit start code {next}{second}{third}");
            chars.next();
            chars.next();
        } else {
            let mut iter_strings = Vec::new();

            let second = match chars.next() {
                Some('3') => {
                    #[cfg(debug_assertions)]
                    println!("second was 3 when searching");
                    finished = true;
                    'N'
                }
                Some(second) => second,
                None => {
                    #[cfg(debug_assertions)]
                    println!("second was missing when searching");
                    finished = true;
                    'N'
                }
            };

            let third = match chars.next() {
                Some('3') => {
                    #[cfg(debug_assertions)]
                    println!("third was 3 when searching");
                    finished = true;
                    'N'
                }
                Some(third) => third,
                None => {
                    #[cfg(debug_assertions)]
                    println!("third was missing when searching");
                    finished = true;
                    'N'
                }
            };

            for acid in possible_chains(next, second, third)? {
                let acid_letter = acid.finalise()?;

                let Some(letter) = acid_letter else {
                    looking_for_start_code = true;
                    #[cfg(debug_assertions)]
                    println!("Hit stop code");
                    strings.dedup();
                    strings.sort();
                    possibilities.push(strings);
                    strings = Vec::new();
                    continue 'outer;
                };

                if strings.is_empty() {
                    iter_strings.push(String::from(letter));
                } else {
                    for mut string in strings.clone() {
                        string.push(letter);
                        iter_strings.push(string)
                    }
                }
            }

            strings = iter_strings;
        }
    }

    // Case where there was no stop code at the end
    if !strings.is_empty() {
        strings.dedup();
        strings.sort();
        possibilities.push(strings);
    }

    let mut out_strings = Vec::<String>::new();
    for items in possibilities {
        let mut iter_strings = Vec::new();
        for item in items {
            if out_strings.is_empty() {
                iter_strings.push(item);
            } else {
                for string in &out_strings {
                    iter_strings.push(format!("{string} {item}"));
                }
            }
        }

        if !iter_strings.is_empty() {
            out_strings = iter_strings;
        }
    }

    Ok(out_strings)
}

#[cfg(test)]
mod tests {
    use crate::solver::possibilities;

    #[test]
    fn test() {
        const SEQ: &str = "5ATGATGGARTGGATGGARTGGAAYTAYGCN3";

        assert_eq!(possibilities(SEQ), Ok(vec![String::from("MEWMEWNYA")]))
    }
}