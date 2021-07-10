pub fn add(x: & str) -> usize{
    panic!("Not Implemented");
}
#[cfg(test)]
mod tests{
    use crate::stringcalc::add;

    #[test]
    fn should_add_two_numbers(){
        assert_eq!(3,add("1,2"))
    }
}