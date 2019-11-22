# rsql-rs

**The crate name has been changed to `rsql`, please be notified**

[![Build Status](https://travis-ci.com/UkonnRa/rsql-rs.svg?branch=master)](https://travis-ci.com/UkonnRa/rsql-rs)
![crates.io](https://img.shields.io/crates/v/rsql.svg)

A simple RSQL/FIQL parser written in Rust and [Pest](https://github.com/pest-parser/pest)

## Features

- [x] Decode FIQL/RSQL query into AST
- [x] Basic parsing test
- [x] Encode AST into FIQL/RSQL query
- [x] Better error system
- [x] Register own `Comparison`s

## About RSQL/FIQL

RSQL is a query language for REST APIs. It’s based on [FIQL](https://tools.ietf.org/html/draft-nottingham-atompub-fiql-00) (Feed Item Query Language) – an URI-friendly syntax for expressing filters across the entries in an Atom Feed. FIQL is great for use in URI's. there are no unsafe characters, so URL encoding is not required. On the other side, FIQL’s syntax is not very intuitive and URL encoding isn't necessarily a bad thing, so RSQL also provides a friendlier syntax for logical operators and some of the comparison operators.
