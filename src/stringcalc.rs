pub fn add(x: & str) -> usize{
    let mut split=x.split(',');
    let mut sum = 0;
    for val in split {
        sum = sum + val.parse::<usize>().unwrap_or(0)
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
    #[test]
    fn should_return_same_number_if_single(){
        assert_eq!(1,add("1"))
    }
    #[test]
    fn should_return_0_when_empty(){
        assert_eq!(0,add(""))
    }
    #[test]
    fn should_return_for_4_numbers(){
        assert_eq!(10,add("1,2,3,4"))
    }
}