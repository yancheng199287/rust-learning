#[cfg(test)]
mod tests {

    //test是一个模块，需要引入外部所有模块内容使用
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_works1() {
        assert_eq!(2 + 2, 5);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }


    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}




#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}


pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}