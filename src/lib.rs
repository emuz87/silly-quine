#![feature(result_option_inspect)]
#![feature(int_log)]
use comfy_table::{Table, Row, Cell};
use itertools::{Itertools, EitherOrBoth};
use std::hash::Hash;
use std::fmt;
use std::{ops::Deref, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Digits { One, Zero, Blnk }
impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Digits::Blnk => "_",
            Digits::Zero => "0",
            Digits::One => "1"
        })
    }
}
impl TryFrom<char> for Digits {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Digits::One),
            '0' => Ok(Digits::Zero),
            '_' => Ok(Digits::Blnk),
            _ => Err("Invalid character")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Num(Vec<Digits>);
impl Num {
    fn join(&self, other: &Num) -> Option<Self> {
        let mut n = 0;
        let new = self.0.iter().rev().zip_longest(other.0.iter().rev()).rev().fold(Vec::new(), |mut acc, eob| {
            let (x, y) = match eob {
                EitherOrBoth::Both(x, y) => (x, y),
                EitherOrBoth::Left(x) => (x, &Digits::Zero),
                EitherOrBoth::Right(y) => (&Digits::Zero, y)
            };
            acc.push(if x == y { *x } else { n += 1; Digits::Blnk }); acc});
        if n == 1 { Some(Num(new)) } else { None }
    }
    fn ones(&self) -> u8 {
        self.iter().fold(0, |acc, x| if let Digits::One = x { acc+1 } else { acc })
    }
}
impl TryFrom<String> for Num {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut error = false;
        Ok(Num(value.chars().map(|c| Digits::try_from(c).unwrap_or_else(|_| {error = true; Digits::Blnk})).collect()))
    }
}
impl From<u8> for Num {
    fn from(n: u8) -> Self {
        Num(format!("{:b}", n).chars().map(|c| Digits::try_from(c).unwrap()).collect())
    }
}
impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x.to_string()); acc.push(' '); acc}))
    }
}
impl Deref for Num {
    type Target = Vec<Digits>;
    fn deref(&self) -> &Self::Target {&self.0}
}

#[derive(Clone, PartialEq)]
pub struct MintermTable {
    minterms: Vec<u8>,
    table: Vec<HashSet<(Vec<u8>, Num)>>,
}
// callan
impl MintermTable {
    fn reduce(self) -> Self {
        let mut table = Vec::<HashSet<(Vec<u8>, Num)>>::new();
        let mut iter = self.table.iter().peekable();
        while let Some(nos) = iter.next() {
            table.push(nos.iter()
                .fold(HashSet::new(), |mut acc, (m, c)| { iter.peek()
                    .inspect(|nnos| nnos.iter().for_each(|(nm, nc)| {c.join(&nc)
                        .inspect(|nnc| {acc.insert((m.into_iter()
                            .chain(nm.into_iter()).sorted().map(|x|*x).collect::<Vec<u8>>(), nnc.clone()));});})); acc}));
        }
        self.table.iter().enumerate().for_each(|(i, s)| s.iter().for_each(|(m, c)|
            if !table.iter().flatten().any(|(nm, _)| m.iter().all(|x| nm.contains(x))) {
                table[i].insert((m.clone(), c.clone()));
            }));
        MintermTable { table, minterms: self.minterms }
    }
    pub fn reduce_all(self) -> Self {
        let mut new = self;
        let mut oldc = new.table.iter().flatten().count();
        new = new.reduce();
        let mut newc = new.table.iter().flatten().count();
        while newc != oldc {
            println!("iter");
            oldc = newc;
            new = new.reduce();
            newc = new.table.iter().flatten().count();
        }
        new
    }
}
impl From<BinaryFunction> for MintermTable {
    fn from(bf: BinaryFunction) -> Self {
        let mut table = (0..=bf.0.iter().sorted().last().unwrap().ilog2()+1).fold(Vec::new(), |mut acc, _| {acc.push(HashSet::new()); acc});
        bf.0.iter().map(|x| (x, Num::from(*x))).for_each(|(m, c)| {table.get_mut(c.ones() as usize).unwrap().insert((vec![*m], c));});
        MintermTable { table, minterms: bf.0 }
    }
}
impl fmt::Display for MintermTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table
            .set_header(vec!["Min Values", "Implicants"])
            .add_rows(self.table.iter().flatten().map(|(m, c)| (Cell::new(m.into_iter().fold(String::new(), |mut acc, x|
                {acc.push_str(&x.to_string()); acc.push(' '); acc})), Cell::new(c.to_string())))
                    .fold(vec![], |mut v, (m, c)| {let mut row = Row::new();
                        row.add_cell(m); row.add_cell(c); v.push(row); v}));
        write!(f, "{table}")
    }
}

pub struct ImplicantTable {
    minterms: Vec<u8>,
    table: Vec<(Num, HashSet<u8>)>
}
impl fmt::Display for ImplicantTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        table.set_header(self.minterms.iter().fold(vec!["X".to_string()],
            |mut acc, x| {acc.push(x.to_string()); acc}))
        .add_rows(self.table.iter().fold(Vec::<Row>::new(), |mut acc, (n, m)| {acc.push(self.minterms.iter()
            .fold(vec![Cell::new(n)], |mut accc, x| 
                {if m.contains(x) {accc.push(Cell::new("Y"))} else {accc.push(Cell::new(" "))}; accc}).into()); acc}));

        write!(f, "{table}")
    }
}
impl From<MintermTable> for ImplicantTable {
    fn from(mt: MintermTable) -> Self {
        Self { 
            table: mt.table.into_iter().flatten().fold(Vec::<(Num, HashSet<u8>)>::new(),
                |mut acc, (m, c)| {acc.push((c, HashSet::from_iter(m.into_iter()))); acc}),
            minterms: mt.minterms
        }
    }
}

pub struct BinaryFunction(pub Vec<u8>);