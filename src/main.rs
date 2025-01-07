use std::fmt;
use std::fmt::Display;
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
struct TreeNode {
    value: i128,
    method: Method,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
impl TreeNode {
    //我好像使用的是先序遍历来实现从左到右计算
    fn solve(self, i: i128, m: Method) -> i128 {
        fn calc(a: i128, b: i128, m: Method) -> i128 {
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
            let l = self.left.unwrap().solve(0, Method::Add);
            let res = calc(i, l, m); //res是左和上级左的和
            let r = self.right.unwrap().solve(res, self.method);
            return r;
        }
    }
    fn zero()->Self{
        TreeNode {
            value: 0,
            method: Method::Direct,
            left: None,
            right: None,
        }
    }
}
impl Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {:?})", self.value, self.method)
    }
}
/*
通过level表明父级优先级，如果父级级别比当前低，那么需要让自己优先计算，方法就是把自己放到左节点中
*/
fn parse(s: &str, e: Element, level: u8) -> (usize, TreeNode) {
    //println!("尝试解析{} 作为 {:?}", s, e);
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
                    if temp.parse::<u128>().is_ok() {
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
                    value: if neg { -1 } else { 1 }
                        * (&s[if neg { 1 } else { 0 }..i - 1])
                            .parse::<i128>()
                            .unwrap(),
                    method: Method::Direct,
                    left: None,
                    right: None,
                },
            );
        }
        Element::Expression => {
            //要么是数字/括号-符号-表达式 要么是数字

            let len: usize;
            let l_number: TreeNode;
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
                            value: 0,
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
                                value: 0,
                                method: m,
                                left: Some(Box::new(l_number)),
                                right: Some(Box::new(r_exp)),
                            },
                        );
                    } else if level == 0 {
                        return (
                            0,
                            TreeNode {
                                value: 0,
                                method: Method::Add,
                                left: Some(Box::new(TreeNode {
                                    value: 0,
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
                    value: 0,
                    method: Method::Add,
                    left: Some(Box::new(inside)),
                    right: Some(Box::new(TreeNode::zero())),
                },
            );
        }
    }
}
fn calc(s:&str)->i128{
    let (_,res)=parse(s, Element::Expression, 0);
    return res.solve(0,Method::Add);
}
fn test(){
    assert_eq!(calc("1+1"),2);
    assert_eq!(calc("1-1"),0);
    assert_eq!(calc("1*1"),1);
    assert_eq!(calc("1/1"),1);
    assert_eq!(calc("1-(1-2)"),2);
    assert_eq!(calc("1*(1-3)*(8+3)"),-22);
    assert_eq!(calc("4/3"),1);
    assert_eq!(calc("(3+4)*3/(4+2)"),3);



}
fn main() {
    // let mut s=String::from("0");
    // for i in 1..25000{
    //     s+=&format!("+{}",i);
    // }
    test();

    let a = "(-2)*(3)";
    let r=calc(a);
    println!("结果:{}", r);
}
