use num::FromPrimitive;
use std::fmt;
use std::fmt::Display;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug)]
enum Element {
    Number,
    Expression,
    Bracket,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum Method {
    Add,
    Sub,
    Mul,
    Div,
    Direct,
}
//定义Arithmetic是拥有xxx特征的特征
trait Arithmetic:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Display
    + std::str::FromStr
    + FromPrimitive
{
}
//定义所有实现了这些特征的类型都是Arithmetic
impl<T> Arithmetic for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Display
        + std::str::FromStr
        + FromPrimitive
{
}

struct TreeNode<T: Arithmetic> {
    value: T,
    method: Method,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
impl<T: Arithmetic> TreeNode<T> {
    //我好像使用的是先序遍历来实现从左到右计算
    fn solve(self, i: T, m: Method) -> T {
        fn calc<T: Arithmetic>(a: T, b: T, m: Method) -> T {
            #[cfg(debug_assertions)]
            println!("计算:{} {:?} {}", a, m, b);
            match m {
                Method::Add => a + b,
                Method::Div => a / b,
                Method::Mul => a * b,
                Method::Sub => a - b,
                Method::Direct => panic!("这个地方不可以Direct"),
            }
        }

        if self.method == Method::Direct {
            return calc(i, self.value, m);
        } else {
            let l = self
                .left
                .unwrap()
                .solve(T::from_i128(0).unwrap(), Method::Add);
            let res = calc(i, l, m); //res是左和上级左的和
            let r = self.right.unwrap().solve(res, self.method);
            return r;
        }
    }
    fn zero() -> Self {
        TreeNode {
            value: T::from_i128(0).unwrap(),
            method: Method::Direct,
            left: None,
            right: None,
        }
    }
}
impl<T: Arithmetic> Display for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {:?})", self.value, self.method)
    }
}
/*
通过level表明父级优先级，如果父级级别比当前低，那么需要让自己优先计算，方法就是把自己放到左节点中
*/
fn parse<T: Arithmetic>(s: &str, e: Element, level: u8) -> (usize, TreeNode<T>) {
    #[cfg(debug_assertions)]
    println!("尝试解析{} 作为 {:?}", s, e);
    match e {
        Element::Number => {
            let mut i = 1;
            let mut neg = false;
            loop {
                if i != s.len() + 1 {
                    let temp = &s[neg as usize..i];
                    if temp == "-" {
                        i += 1;
                        neg = true;
                        continue;
                    }
                    if temp.parse::<f64>().is_ok() {
                        i += 1;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            return (
                i - 1,
                TreeNode {
                    value: if neg {
                        T::from_i128(-1).unwrap()
                    } else {
                        T::from_i128(1).unwrap()
                    } * (&s[if neg { 1 } else { 0 }..i - 1])
                        .parse::<T>()
                        .unwrap_or(T::from_i128(0).unwrap()),
                    method: Method::Direct,
                    left: None,
                    right: None,
                },
            );
        }
        Element::Expression => {
            //要么是数字/括号-符号-表达式 要么是数字

            let len: usize;
            let l_number: TreeNode<T>;
            //看看是不是括号
            if s.as_bytes()[0] == b'(' {
                (len, l_number) = parse(s, Element::Bracket, 0);
            } else {
                (len, l_number) = parse(s, Element::Number, 0);
                //然后就开始解析数字
            }
            //检查一下后面还有没有东西
            if len == s.len() {
                //没有了 那么这个表达式就是一个单纯的数字而已
                return (len, l_number);
            }
            let m: Method;
            //然后接下来是一个符号
            match &s[len..len + 1] {
                "+" => {
                    m = Method::Add;
                }
                "-" => {
                    m = Method::Sub;
                }
                "*" => {
                    m = Method::Mul;
                }
                "/" => {
                    m = Method::Div;
                }
                _ => {
                    panic!("什么鬼符号{}", s)
                }
            }
            match m {
                Method::Add | Method::Sub => {
                    let (_, r_exp) = parse(&s[len + 1..], Element::Expression, 0);
                    return (
                        0,
                        TreeNode {
                            value: T::from_i128(0).unwrap(),
                            method: m,
                            left: Some(Box::new(l_number)),
                            right: Some(Box::new(r_exp)),
                        },
                    );
                }
                Method::Mul | Method::Div => {
                    let (_, r_exp) = parse(&s[len + 1..], Element::Expression, 1);
                    if level == 1 {
                        return (
                            0,
                            TreeNode {
                                value: T::from_i128(0).unwrap(),
                                method: m,
                                left: Some(Box::new(l_number)),
                                right: Some(Box::new(r_exp)),
                            },
                        );
                    } else if level == 0 {
                        return (
                            0,
                            TreeNode {
                                value: T::from_i128(0).unwrap(),
                                method: Method::Add,
                                left: Some(Box::new(TreeNode {
                                    value: T::from_i128(0).unwrap(),
                                    method: m,
                                    left: Some(Box::new(l_number)),
                                    right: Some(Box::new(r_exp)),
                                })),
                                right: Some(Box::new(TreeNode::zero())),
                            },
                        );
                    } else {
                        unreachable!()
                    }
                }
                Method::Direct => unreachable!(),
            }
        }
        Element::Bracket => {
            //如果首字不是'('那么报错
            if s.as_bytes()[0] != b'(' {
                unreachable!()
            }
            //括号的内部是表达式
            let matched_bracket = s.find(')').unwrap();
            if matched_bracket == 1 {
                panic!("这tm空括号？")
            }
            let (_, inside) = parse(&s[1..matched_bracket], Element::Expression, 0);
            return (
                matched_bracket + 1,
                TreeNode {
                    value: T::from_i128(0).unwrap(),
                    method: Method::Add,
                    left: Some(Box::new(inside)),
                    right: Some(Box::new(TreeNode::zero())),
                },
            );
        }
    }
}
//如果仅使用整数计算 那么就替换成这个函数
#[warn(dead_code)]
fn calc_i128(s: &str) -> i128 {
    let (_, res) = parse(s, Element::Expression, 0);
    return res.solve(0, Method::Add);
}
fn calc_f64(s: &str) -> f64 {
    let (_, res) = parse(s, Element::Expression, 0);
    return res.solve(0.0, Method::Add);
}
fn _test() {
    assert_eq!(calc_i128("1+1"), 2);
    assert_eq!(calc_i128("1-1"), 0);
    assert_eq!(calc_i128("1*1"), 1);
    assert_eq!(calc_i128("1/1"), 1);
    assert_eq!(calc_i128("1-(1-2)"), 2);
    assert_eq!(calc_i128("1*(1-3)*(8+3)"), -22);
    assert_eq!(calc_i128("4/3"), 1);
    assert_eq!(calc_i128("(3+4)*3/(4+2)"), 3);
}

fn main()->! {

    //_test();

    loop{
        let mut line=String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        let exp=line.lines().next().unwrap();
        if exp.is_empty(){
            continue;
        }
        let result=calc_f64(exp);          //对每行遍历
        println!("--> {}\n",result);
    }
}
