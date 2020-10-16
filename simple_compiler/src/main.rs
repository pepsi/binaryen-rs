extern crate pest;
#[macro_use]
extern crate pest_derive;

use binaryen_rs::*;
use pest::Parser;
#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

fn parse_pair(p: i32){

}

fn main() {
    let mut module = Module::new();
    let input = std::fs::read("in.sc").unwrap();
    let pairs: ::pest::iterators::Pairs<Rule> = IdentParser::parse(
        Rule::math,
        std::str::from_utf8(&input).unwrap_or_else(|e| panic!("{}", e)),
    ).unwrap();
    
    let a = 5;
    a = 6;
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        let mut parts: Vec<&str> = vec![];

        // for inner_pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::math => {
                // println!("math op -> pairs = {:?}", pairs.next());
                let mut pairs = pair.into_inner();

                let lhs: &str = pairs.next().unwrap().as_str();
                
                dbg!(lhs);
                let op: &str = pairs.next().unwrap().as_str();
                dbg!(op);
                let rhs: &str = pairs.next().unwrap().as_str();
                dbg!(rhs);
                let br_op = match op {
                    "+" => Op::add_int32(),
                    "-" => Op::sub_int32(),
                    "*" => Op::mul_int32(),

                    _ => unimplemented!(),
                };
                let br_lhs = Literal::int_32(lhs.parse::<i32>().unwrap());
                let br_rhs = Literal::int_32(rhs.parse::<i32>().unwrap());
                let const_lhs = module.make_const(br_lhs);
                let const_rhs = module.make_const(br_rhs);

                let body = module.binary(br_op, const_lhs, const_rhs);
                module.add_function("Test", Type::none(), Type::int_32(), vec![], body);
            }
            Rule::digit => println!("digit"),
            Rule::number => println!("number"),

            _ => println!("somethinh weird is happening"),
        };
        // }
        println!("parts = {:?}", parts);

        // println!("Left = {}\nop = {}\nright = {}", parts.get(0).unwrap(), parts.get(1).unwrap(), parts.get(2).unwrap());
    }
    module.print()
}
