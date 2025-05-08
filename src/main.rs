// 写一个简单的乘法运算器
// 1. 定义一个函数，接受两个整数参数，返回它们的乘积
// 2. 在主函数中调用这个函数，并打印结果
// 3. 使用 Rust 的错误处理机制，处理可能的错误情况，比如参数为负数时返回错误
// 4. 使用 Cargo 创建一个新的 Rust 项目，并在项目中实现这个功能
// 5. 使用 Rust 的测试框架，编写单元测试，测试乘法函数的正确性
// 6. 使用 Rust 的文档注释，给函数添加文档说明
// 7. 使用 Rust 的格式化输出，打印函数的调用结果
// 8. 使用 Rust 的命令行参数，接受用户输入的两个整数，并调用乘法函数
// 9. 使用 Rust 的日志库，记录函数的调用过程
// 10. 使用 Rust 的多线程，异步调用乘法函数，并打印结果
use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <num1> <num2>", args[0]);
        process::exit(1);
    }

    let num1 = args[1].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[1]);
        process::exit(1);
    });

    let num2 = args[2].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", args[2]);
        process::exit(1);
    });

    let result = multiply(num1, num2).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    println!("{} * {} = {}", num1, num2, result);

    // 异步调用乘法函数
    let handle = thread::spawn(move || {
        let result = multiply(num1, num2).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });
        println!("Async result: {} * {} = {}", num1, num2, result);
    });

    // 等待异步任务完成
    handle.join().unwrap();
}

fn multiply(num1: i32, num2: i32) -> Result<i32, String> {
    if num1 < 0 || num2 < 0 {
        return Err("Negative numbers are not allowed".to_string());
    }

    // 模拟耗时操作
    thread::sleep(Duration::from_secs(1));

    Ok(num1 * num2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3).unwrap(), 6);
        assert_eq!(multiply(0, 5).unwrap(), 0);
        assert_eq!(
            multiply(-1, 5).unwrap_err(),
            "Negative numbers are not allowed"
        );
        assert_eq!(
            multiply(1000000000, 2).unwrap_err(),
            "Negative numbers are not allowed"
        );
    }
}
