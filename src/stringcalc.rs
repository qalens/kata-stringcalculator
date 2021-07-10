pub fn add(numbers: & str) -> usize{
    let mut dels :Vec<String>= vec![",".to_string()];
    let mut x = numbers.clone();

    let mut split=x.split('\n');
    let mut sum = 0;
    let mut negatives : Vec<String>=vec![];
    let mut index = 0;

    for line in split {
        if(line.starts_with("//") && index == 0){
            dels.pop();
            let newstrs = line.split("][");
            for new_str in newstrs {
                let delim = new_str.replace("//","").replace("[","").replace("]","");
                dels.push(format!("{0}",delim));
            }

            continue;
        }
        let mut new_line:String= line.to_string();
        for del in &dels {
            new_line = format!("{0}",new_line.replace(del,","));
        }
        for val in new_line.split(','){
            if(val.starts_with('-')){
                negatives.push(format!("{0}",val));
            }
            let num = val.parse::<usize>().unwrap_or(0);
            if num<=1000 {
                sum = sum + num;
            }
        }
        index = index+1;
    }
    if(!negatives.is_empty()){
        panic!(format!("negatives not allowed {0}",negatives.join(" ")));
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

    #[should_panic(expected = "negatives not allowed -1 -3 -5")]
    #[test]
    fn should_throw_exception_when_negative_numbers_are_present(){
        add("-1,2,-3,4,-5");
    }
    #[test]
    fn should_ignore_number_bigger_than_1000(){
        assert_eq!(2,add("2,1001"))
    }
    #[test]
    fn should_support_delimiter_of_any_length(){
        assert_eq!(6,add("//[***]\n1***2***3"));
    }
    #[test]
    fn should_support_multiple_delimiter(){
        assert_eq!(6,add("//[*][%]\n1*2%3"));
    }
}