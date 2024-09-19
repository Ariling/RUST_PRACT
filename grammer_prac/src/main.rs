fn main() {
    // mut을 붙이지 않으면 불변성을 가지므로 에러가 뜬다.
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
    // const는 상수로서 mut와 함께 사용할 수 없으므로 항상 불변이다. 또한 값의 타입이 반드시 명시되어야 할 것

    // 섀도잉 관련
    let x = 5;

    let x = x + 1;

    // scope를 알면 어떤 느낌인지 대략적으로 알 듯!
    // 안의 x는 12를 return하지만 바깥은 6을 return한다.

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    // 데이터 타입 중 함수 부분만
    another_function("function");
    // 이거는 return을 표시할 때 값을 리턴할 때는 세미콜론을 붙이지 않는다.
    let y = plus_one(x);
    println!("The value of y is : {y}");

    // 문자열 관련 코드
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn another_function(x : &str){
    println!("The value of x is : {x}");
}

fn plus_one(x : i32) -> i32 {
    x+1
}
