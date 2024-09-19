fn main() {
    let s = String::from("hello"); //s가 스코프 안으로 들어온다.
    // 함수로 s의 값 이동 -> 더 이상 유효하지 않게 된다.
    takes_ownership(s);
    // 이렇게 그냥 쓰면 에러가 일어난다.
    // println!("{}",s);
    let x = 5;
    // x가 함수로 이동되지만 Copy이므로 앞으로 계속 x를 사용해도 괜찮다.
    makes_copy(x);

    // 참조자를 매개변수로 하면 소유권은 유지되어 프린트를 해도 된다.
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 가변 참조자
    let mut s2 = String::from("hello");

    change(&mut s2);

    let s3 = String::from("hello world");
    let hello = &s3[0..5];
    // let len = s3.len();
    // let world = &s3[6..]; 로 가능하다.
    let world = &s3[6..11];

    println!("{}, {}", hello, world);
    // 배열을 슬라이스로 참조하고 싶다면 다음과 같이 가능
    // let slice = &a[1..3]
    // assert_eq!(slice, &[2,3]);

}

fn takes_ownership(some_string : String){
    println!("{}", some_string);
} // some_string이 스코프 밖으로 벗어나고 drop이 호출되면서 메모리 해체
// 다시 돌려 받고 싶다면 return문을 써야 한다.

fn makes_copy(some_integer : i32){
    println!("{}", some_integer);
} // 스코프 밖으로 벗어나지만 별다른 일이 일어나지 않는다.

fn calculate_length(s : &String) -> usize {
    s.len()
}

fn change(some_string : &mut String){
    some_string.push_str(", world");
}