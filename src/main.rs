fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("{}", spaces)

}


/*
mut과 shadowing의 차이는

shadowing을 이용한 let은 값의 유형도 변경 가능,
mut은 값의 유형 불가

다음과 같은 예

// 가능
let spaces = "       ";
let spaces = spaces.len();


// 컴파일 에러
let mut spaces = "       ";
spaces = spaces.len();
*/