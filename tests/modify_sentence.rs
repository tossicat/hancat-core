use hancat_core::modify_sentence;

#[test]
fn 조사_처리() {
    let result = modify_sentence("{철수, 이} 왔다.").unwrap();
    assert_eq!(result, "철수가 왔다.");
}

#[test]
fn 조사_여러개() {
    let result = modify_sentence("{철수, 이} {밥, 을} 먹었다.").unwrap();
    assert_eq!(result, "철수가 밥을 먹었다.");
}

#[test]
fn 용언_활용() {
    let result = modify_sentence("여기서 {쉬다, 세요}.").unwrap();
    assert_eq!(result, "여기서 쉬세요.");
}

#[test]
fn 조사와_용언_통합() {
    let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.").unwrap();
    assert_eq!(result, "철수가 밥을 먹었습니다.");
}

#[test]
fn 다로_끝나는_명사_조사처리() {
    let result = modify_sentence("{바다, 을} 건넜다.").unwrap();
    assert_eq!(result, "바다를 건넜다.");
}

#[test]
fn 패턴_없는_문장() {
    let result = modify_sentence("안녕하세요.").unwrap();
    assert_eq!(result, "안녕하세요.");
}

#[test]
fn 닫는_중괄호_없음() {
    let result = modify_sentence("{철수, 이");
    assert!(result.is_err());
}

#[test]
fn 쉼표_없음() {
    let result = modify_sentence("{철수 이}");
    assert!(result.is_err());
}

#[test]
fn 빈_단어() {
    let result = modify_sentence("{, 이}");
    assert!(result.is_err());
}

#[test]
fn 빈_접사() {
    let result = modify_sentence("{철수, }");
    assert!(result.is_err());
}
