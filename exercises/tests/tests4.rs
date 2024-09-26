// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn negative_width() {
        let result = panic::catch_unwind(|| {
            Rectangle::new(-10, 10);
        });

        assert!(result.is_err()); // 检查是否发生了 panic
        if let Err(err) = result {
            // 使用 downcast_ref 直接查找具体的错误信息
            let message = err
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| err.downcast_ref::<&str>().map(|s| *s));
                
            assert_eq!(message, Some("Rectangle width and height cannot be negative!"));
        }
    }

    #[test]
    fn negative_height() {
        let result = panic::catch_unwind(|| {
            Rectangle::new(10, -10);
        });

        assert!(result.is_err()); // 检查是否发生了 panic
        if let Err(err) = result {
            let message = err
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| err.downcast_ref::<&str>().map(|s| *s));
                
            assert_eq!(message, Some("Rectangle width and height cannot be negative!"));
        }
    }
}
