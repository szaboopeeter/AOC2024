use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Look-around regex doesn't seem to work with regex::Regex, so doing tiny state-machine-y approach
    let re = Regex::new(r"(mul\(([0-9]*}),([0-9]*})\))|(do\(\))|(don't\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for caps in re.captures_iter(input) {
        match &caps[0][..3] {
            "mul" => {
                if enabled {
                    let a = caps[2].parse::<i32>().unwrap();
                    let b = caps[3].parse::<i32>().unwrap();
                    sum += a * b;
                }
            }
            "do(" => {
                enabled = true;
            }
            "don" => {
                enabled = false;
            }
            _ => {}
        }
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
