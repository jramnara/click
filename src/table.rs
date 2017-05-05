// Copyright 2017 Databricks, Inc.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Stuff to handle outputting a table of resources, including
/// applying filters and sorting

use clap::ArgMatches;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::{format, Table};
use regex::Regex;

enum CellSpecTxt<'a> {
    Index,
    Str(&'a str),
    String(String),
}

/// Holds a specification for a prettytable cell
pub struct CellSpec<'a> {
    txt: CellSpecTxt<'a>,
    pub style: Option<&'a str>,
    pub align: Option<format::Alignment>,
}

impl<'a> CellSpec<'a> {
    pub fn new(txt: &'a str) -> CellSpec<'a> {
        CellSpec {
            txt: CellSpecTxt::Str(txt),
            style: None,
            align: None,
        }
    }

    pub fn new_owned(txt: String) -> CellSpec<'a> {
        CellSpec {
            txt: CellSpecTxt::String(txt),
            style: None,
            align: None,
        }
    }

    pub fn new_index() -> CellSpec<'a> {
        CellSpec {
            txt: CellSpecTxt::Index,
            style: None,
            align: None,
        }
    }

    pub fn with_style(txt: &'a str, style: &'a str) -> CellSpec<'a> {
        CellSpec {
            txt: CellSpecTxt::Str(txt),
            style: Some(style),
            align: None,
        }
    }

    pub fn with_style_owned(txt: String, style: &'a str) -> CellSpec<'a> {
        CellSpec {
            txt: CellSpecTxt::String(txt),
            style: Some(style),
            align: None,
        }
    }

    pub fn to_cell(&self, index: usize) -> Cell {
        let cell = match self.txt {
            CellSpecTxt::Index => Cell::new(format!("{}", index).as_str()),
            CellSpecTxt::Str(ref s) => Cell::new(s),
            CellSpecTxt::String(ref s) => Cell::new(s.as_str()),
        };

        if let Some(style) = self.style {
            cell.style_spec(style)
        } else {
            cell
        }
    }

    pub fn matches(&self, regex: &Regex) -> bool {
        match self.txt {
            CellSpecTxt::Index => false,
            CellSpecTxt::Str(ref s) => regex.is_match(s),
            CellSpecTxt::String(ref s) => regex.is_match(s),
        }
    }
}

pub fn get_regex(matches: &ArgMatches) -> Result<Option<Regex>, String> {
    match matches.value_of("regex") {
        Some(pattern) => {
            if let Ok(regex) = Regex::new(pattern) {
                Ok(Some(regex))
            }
            else {
                Err(format!("Invalid regex: {}", pattern))
            }
        }
        None => Ok(None),
    }
}

pub fn filter<'a, T, I> (things: I, regex: Regex) -> Vec<(T, Vec<CellSpec<'a>>)>
    where I: Iterator<Item=(T,Vec<CellSpec<'a>>)> {
    things.filter_map(|thing| {
        let mut has_match = false;
        for cell_spec in thing.1.iter() {
            if !has_match {
                has_match = cell_spec.matches(&regex);
            }
        }
        if has_match {
            Some(thing)
        } else {
            None
        }
    }).collect()
}

pub fn add_to_table<'a, T>(table: &mut Table, specs: &Vec<(T, Vec<CellSpec<'a>>)>) {
    for (index, t_spec) in specs.iter().enumerate() {
        let row_vec: Vec<Cell> = t_spec.1.iter().map(|spec| spec.to_cell(index)).collect();
        table.add_row(Row::new(row_vec));
    }
}
