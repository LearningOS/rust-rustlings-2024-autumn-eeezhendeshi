// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.
struct Wrapper<T> {
    value: T,
}

enum An<'a> {
    A(u32),
    B(&'a str),
}

impl<'a> Wrapper<An<'a>> {
    pub fn new(value: An<'a>) -> Wrapper<An<'a>> {
        Wrapper { value }
    }

    pub fn unwrap(&self) -> u32 {
        match &self.value {
            An::A(v) => *v,
            An::B(s) => s.parse().unwrap_or(0), // 默认返回 0，如果解析失败
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        let wrapper = Wrapper::new(An::A(42));
        assert_eq!(wrapper.unwrap(), 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let wrapper = Wrapper::new(An::B("123"));
        assert_eq!(wrapper.unwrap(), 123);
        
        let wrapper_invalid = Wrapper::new(An::B("Foo"));
        assert_eq!(wrapper_invalid.unwrap(), 0); // 解析失败返回 0
    }
}
