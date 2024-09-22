// 보편적인 방식으로 use를 활용하여 스코프로 가져오기
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
    // 이름이 같은 두 개의 타입을 동일한 스코프에 가져오려면 부모 모듈을 반드시 명시하기!
    // 예를 들어 fmt나 io에 둘 다 Result타입을 구분하는 경우 fmt::Result, io::Result 이런 식으로 명시하기
    // glob 연산자 *를 하면 모든 공개아이템을 가져올 수 있다. 
}
