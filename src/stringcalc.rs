pub fn add(numbers: & str) -> usize{
    let mut del: char = ',';
    let mut x = numbers.clone();
    if(x.starts_with("//")){
        del = x.chars().nth(2).unwrap();
        x = &numbers[4..];
    }
    let mut split=x.split('\n');
    let mut sum = 0;

    for line in split {
        for val in line.split(del){
            sum = sum + val.parse::<usize>().unwrap_or(0)
        }
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
    #[test]
    fn should_return_sum_when_new_line_present(){
        assert_eq!(6,add("1\n2,3"))
    }
    #[test]
    fn should_support_dynamic_delimiter(){
        assert_eq!(3,add("//;\n1;2"))
    }
}