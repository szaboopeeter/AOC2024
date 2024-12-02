pub fn is_safe(level: Vec<i32>) -> bool {
    let all_inc = level
        .windows(2)
        .all(|w| w[0] - w[1] > 0 && w[0] - w[1] <= 3);
    let all_dec = level
        .windows(2)
        .all(|w| w[1] - w[0] > 0 && w[1] - w[0] <= 3);
    return all_inc || all_dec;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    for line in input.lines() {
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(items) {
            result += 1;
        }
    }
    Ok(result.to_string())
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
