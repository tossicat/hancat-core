use hancat_core::modify_sentence;

#[test]
fn 조사_처리() {
    let result = modify_sentence("{철수, 이} 왔다.");
    assert_eq!(result, "철수가 왔다.");
}

#[test]
fn 조사_여러개() {
    let result = modify_sentence("{철수, 이} {밥, 을} 먹었다.");
    assert_eq!(result, "철수가 밥을 먹었다.");
}

#[test]
fn 용언_활용() {
    let result = modify_sentence("여기서 {쉬다, 세요}.");
    assert_eq!(result, "여기서 쉬세요.");
}

#[test]
fn 조사와_용언_통합() {
    let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.");
    assert_eq!(result, "철수가 밥을 먹었습니다.");
}

#[test]
fn 다로_끝나는_명사_조사처리() {
    let result = modify_sentence("{바다, 을} 건넜다.");
    assert_eq!(result, "바다를 건넜다.");
}

#[test]
fn 패턴_없는_문장() {
    let result = modify_sentence("안녕하세요.");
    assert_eq!(result, "안녕하세요.");
}

// 파싱 에러 테스트

#[test]
fn 닫는_중괄호_없음_e01() {
    let result = modify_sentence("{철수, 이");
    assert_eq!(result, "{E01}");
}

#[test]
fn 쉼표_없음_e02() {
    let result = modify_sentence("{철수 이}");
    assert_eq!(result, "{E02}");
}

#[test]
fn 빈_단어_e03() {
    let result = modify_sentence("{, 이}");
    assert_eq!(result, "{E03}");
}

#[test]
fn 빈_접사_e04() {
    let result = modify_sentence("{철수, }");
    assert_eq!(result, "{E04}");
}

// 처리 에러 테스트

#[test]
fn 잘못된_조사_e11() {
    let result = modify_sentence("{철수, 잘못된조사}");
    assert_eq!(result, "{E11}");
}

// 에러가 있어도 나머지는 정상 처리

#[test]
fn 부분_에러_처리() {
    let result = modify_sentence("{철수, 이} {, 을} {밥, 을} 먹었다.");
    assert_eq!(result, "철수가 {E03} 밥을 먹었다.");
}
