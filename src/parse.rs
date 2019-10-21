use crate::*;
use pest::{iterators::Pair, Parser};
use std::ops::RangeBounds;

#[derive(Parser)]
#[grammar = "location_formats.pest"]
struct PestParser;

fn check_range<R: RangeBounds<f64>>(value: f64, range: R) -> Result<(), Error> {
    if value.is_nan() || !range.contains(&value) {
        Err(Error::OutOfRange)
    } else {
        Ok(())
    }
}

fn signed_decimal_degrees_to_f64(pair: Pair<Rule>) -> f64 {
    let signed_decimal_degrees = pair.into_inner().next().unwrap();
    let signed_float = signed_decimal_degrees.into_inner();

    let mut sign = 1.0;

    for sign_float in signed_float {
        match sign_float.as_rule() {
            Rule::sign => {
                if "-" == sign_float.as_str() {
                    sign = -1.0
                }
            }
            Rule::float => {
                let f: f64 = sign_float.as_str().parse().unwrap();
                return sign * f;
            }
            _ => unreachable!(),
        }
    }
    unreachable!();
}

fn parse(rule: Rule, s: &str) -> Result<Pair<Rule>, Error> {
    Ok(PestParser::parse(rule, s)
        .map_err(|_| Error::InvalidFormat)?
        .next()
        .unwrap())
}

pub fn parse_latitude(s: &str) -> Result<f64, Error> {
    let parse = parse(Rule::latitude, s)?;

    let lat = match parse.as_rule() {
        Rule::signed_decimal_degrees => signed_decimal_degrees_to_f64(parse),
        _ => unreachable!(),
    };

    check_range(lat, -90.0..=90.0)?;
    Ok(lat)
}

pub fn parse_longitude(s: &str) -> Result<f64, Error> {
    let parse = parse(Rule::longitude, s)?;

    let lon = match parse.as_rule() {
        Rule::signed_decimal_degrees => signed_decimal_degrees_to_f64(parse),
        _ => unreachable!(),
    };

    check_range(lon, -180.0..=180.0)?;
    Ok(lon)
}

pub fn parse_location(s: &str) -> Result<(f64, f64), Error> {
    let parse = parse(Rule::location, s)?;

    let (lat, lon) = match parse.as_rule() {
        Rule::location_signed_decimal_degrees => {
            let mut location_sdd = parse.into_inner();
            let lat_sdd = location_sdd.next().unwrap();
            let lon_sdd = location_sdd.next().unwrap();
            (
                signed_decimal_degrees_to_f64(lat_sdd),
                signed_decimal_degrees_to_f64(lon_sdd),
            )
        }
        _ => unreachable!(),
    };

    check_range(lon, -180.0..=180.0)?;
    check_range(lat, -90.0..=90.0)?;

    Ok((lat, lon))
}
