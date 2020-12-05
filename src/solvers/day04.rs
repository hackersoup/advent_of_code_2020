use std::io::{BufReader, BufRead};
use std::fs::File;
use nom::{
    IResult,
    branch::{
        permutation, alt
    },
    bytes::complete::{
        take_while1, tag
    },
    character::complete::space1,
    combinator::{
        map, opt
    },
    sequence::tuple
};


pub fn solve(input: BufReader<File>) {
    // Not gonna lie, don't really know a better way to do this part...
    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    let lines = lines
        .join(" ");
    let lines = lines
        .split("  ")
        .collect::<Vec<_>>();
    
    let mut counter = 0;
    for line in lines {
        let passport = passport(line);
        match passport {
            Ok((_, _)) => {
                counter += 1;
                println!("--> {}", line);
                println!("---> {:?}", passport);
            },
            Err(e) => {
                //println!("----> {:?}", e);
            }
        }
    }

    println!("[+] Counter: {}", counter);
}

#[derive(Debug)]
pub struct Passport {
    birth_year: usize,
    issue_year: usize,
    exp_year: usize,
    id: usize,
    country_id: Option<usize>,
    haircolor: usize,
    eyecolor: String,
    height: Height
}

#[derive(Debug)]
pub struct Height {
    value: usize,
    unit: HeightUnit
}

#[derive(Debug)]
pub enum HeightUnit {
    Centimeters, Inches
}

/// Parse a Passport structure from a valid input string.
pub fn passport(input: &str) -> IResult<&str, Passport> {

    /*
        So this part right here gets a little janky looking, let me break it down:
        First in the tuple is the standard nom `input` variable representing the next
        text to be parsed after this parser is done executing.
        Next up, we have a series of variables. These will be returned by the
        `permutation` combinator in the order that the parser functions are
        passed in.
        After that are all the `_` representing the whitespace between fields.
        Since a passport has a series of elements sperated by specific whitespace
        elements, we have to add the whitespace parsers into the permutation
        function so that it knows that the whitespaces are valid tokens, but
        we don't need these in order to build the passport struct, so we can
        just throw them away into `_`
    */
    let (input, 
        (birth_year, issue_year, exp_year,
            pass_id, hair_color,
            eye_color, height,
        _, _, _, _, _, _, _,
        country_id)) = permutation((
        passport_birthyear,
        passport_issue_year,
        passport_expiration_year,
        passport_id,
        passport_haircolor,
        passport_eyecolor,
        passport_height,
        single_whitespace, single_whitespace, single_whitespace,
        single_whitespace, single_whitespace, single_whitespace, opt(single_whitespace),
        opt(passport_country_id)

    ))(input)?;

    Ok((input, Passport {
        birth_year: birth_year,
        issue_year: issue_year,
        exp_year: exp_year,
        id: pass_id,
        country_id: country_id,
        haircolor: hair_color,
        eyecolor: String::from(eye_color),
        height: height
    }))
}

fn single_whitespace(input: &str) -> IResult<&str, &str> {
    alt((
        tag(" "),
        tag("\n")
    ))(input)
}

/// Parse arbitrary length decimal number string into a usize.
/// Also pray that it doesn't overflow.
/// TODO: Set a max read size or something
fn take_decimal(input: &str) -> IResult<&str, usize> {
    map(
        take_while1(|c: char| c.is_ascii_digit()),
        |num: &str| num.parse::<usize>().unwrap()
    )(input)
}

fn take_color(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("#")(input)?;
    map(
        take_while1(|c: char| c.is_ascii_hexdigit()),
        |num: &str| usize::from_str_radix(num, 16).unwrap()
    )(input)
}

fn passport_birthyear(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("byr:")(input)?;
    take_decimal(input)
}

fn passport_issue_year(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("iyr:")(input)?;
    take_decimal(input)
}

fn passport_expiration_year(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("eyr:")(input)?;
    take_decimal(input)
}

fn passport_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("pid:")(input)?;
    take_decimal(input)
}

fn passport_country_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("cid:")(input)?;
    take_decimal(input)
}

fn passport_haircolor(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("hcl:")(input)?;
    take_color(input)
}

fn passport_eyecolor(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("ecl:")(input)?;
    take_while1(|c: char| c.is_ascii_alphabetic())(input)
}

fn passport_height_cm(input: &str) -> IResult<&str, Height> {
    let (input, (_, height, _)) = tuple((
        tag("hgt:"),
        take_decimal,
        tag("cm")
    ))(input)?;

    Ok((input, Height { value: height, unit: HeightUnit::Centimeters }))
}

fn passport_height_inches(input: &str) -> IResult<&str, Height> {
    let (input, (_, height, _)) = tuple((
        tag("hgt:"),
        take_decimal,
        tag("in")
    ))(input)?;

    Ok((input, Height { value: height, unit: HeightUnit::Inches }))
}

fn passport_height(input: &str) -> IResult<&str, Height> {
    alt((
        passport_height_cm,
        passport_height_inches
    ))(input)
}