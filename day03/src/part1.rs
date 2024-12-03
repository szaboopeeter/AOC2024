use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r"(mul\(([0-9]*),([0-9]*)\))").unwrap();
    let mut sum = 0;
    for caps in re.captures_iter(input) {
        let a = caps[2].parse::<i32>().unwrap();
        let b = caps[3].parse::<i32>().unwrap();
        sum += a * b;
    }

    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
