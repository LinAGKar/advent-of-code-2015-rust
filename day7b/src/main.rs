use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
enum Source {
    Number(u16),
    Gate(String),
}

#[derive(Debug)]
enum Op {
    Eq(Source),
    Not(Source),
    And(Source, Source),
    Or(Source, Source),
    Rshift(Source, Source),
    Lshift(Source, Source),
}

fn source_value<'a>(gates: &'a HashMap<String, Op>, values: &mut HashMap<&'a str, u16>, source: &'a Source) -> u16 {
    match source {
        &Source::Number(num) => num,
        Source::Gate(name) => wire_value(gates, values, name)
    }
}

fn wire_value<'a>(gates: &'a HashMap<String, Op>, values: &mut HashMap<&'a str, u16>, wire: &'a str) -> u16 {
    values.get(&wire).map(|&x| x).unwrap_or_else(|| {
        let val = match gates.get(wire).unwrap() {
            Op::Eq(source) => source_value(gates, values, source),
            Op::Not(source) => !source_value(gates, values, source),
            Op::And(left_source, right_source) => source_value(gates, values, left_source) &
                                                  source_value(gates, values, right_source),
            Op::Or(left_source, right_source) => source_value(gates, values, left_source) |
                                                 source_value(gates, values, right_source),
            Op::Rshift(left_source, right_source) => source_value(gates, values, left_source) >>
                                                     source_value(gates, values, right_source),
            Op::Lshift(left_source, right_source) => source_value(gates, values, left_source) <<
                                                     source_value(gates, values, right_source),
        };
        values.insert(wire, val);
        val
    })
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = regex::Regex::new(r"(?m)^(?:(?:(\d+)|([a-z]+)) )?(?:([A-Z]+) )?(?:(\d+)|([a-z]+)) -> ([a-z]+)$").unwrap();
    let mut gates: std::collections::HashMap<_, _> = re.captures_iter(&input).map(|caps| {
        let right_source = if let Some(name) = caps.get(4) {
            Source::Number(name.as_str().parse().unwrap())
        } else {
            Source::Gate(caps[5].to_string())
        };

        let left_source = if let Some(name) = caps.get(1) {
            Some(Source::Number(name.as_str().parse().unwrap()))
        } else if let Some(number) = caps.get(2) {
            Some(Source::Gate(number.as_str().to_string()))
        } else {
            None
        };
        
        (caps[6].to_string(), match caps.get(3).map(|x| x.as_str()) {
            Some("NOT") => Op::Not(right_source),
            Some("AND") => Op::And(left_source.unwrap(), right_source),
            Some("OR") => Op::Or(left_source.unwrap(), right_source),
            Some("RSHIFT") => Op::Rshift(left_source.unwrap(), right_source),
            Some("LSHIFT") => Op::Lshift(left_source.unwrap(), right_source),
            _ => Op::Eq(right_source),
        })
    }).collect();

    let initial_a = wire_value(&gates, &mut HashMap::new(), "a");
    gates.insert("b".to_string(), Op::Eq(Source::Number(initial_a)));
    println!("{}", wire_value(&gates, &mut HashMap::new(), "a"));
}
