pub fn add(x: & str) -> usize{
    let mut split=x.split(',');
    let mut sum = 0;
    for val in split {
        sum = sum + val.parse::<usize>().unwrap();
    }
    sum
}
#[cfg(test)]
mod tests{
    use crate::stringcalc::add;

    #[test]
    fn should_add_two_numbers(){
        assert_eq!(3,add("1,2"))
    }
}