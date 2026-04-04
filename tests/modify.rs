use hancat_core::modify;

#[test]
fn 토시_처리() {
    let result = modify("{철수, 이} 왔다.");
    assert_eq!(result, "철수가 왔다.");
}

#[test]
fn 토시_여러개() {
    let result = modify("{철수, 이} {밥, 을} 먹었다.");
    assert_eq!(result, "철수가 밥을 먹었다.");
}

#[test]
fn 용언_활용() {
    let result = modify("여기서 {쉬다, 세요}.");
    assert_eq!(result, "여기서 쉬세요.");
}

#[test]
fn 토시와_용언_통합() {
    let result = modify("{철수, 이} {밥, 을} {먹다, 었습니다}.");
    assert_eq!(result, "철수가 밥을 먹었습니다.");
}

#[test]
fn 다로_끝나는_명사_토시처리() {
    let result = modify("{바다, 을} 건넜다.");
    assert_eq!(result, "바다를 건넜다.");
}

#[test]
fn 패턴_없는_문장() {
    let result = modify("안녕하세요.");
    assert_eq!(result, "안녕하세요.");
}

// 파싱 에러 테스트

#[test]
fn 닫는_중괄호_없음_e01() {
    let result = modify("{철수, 이");
    assert_eq!(result, "{E01}");
}

#[test]
fn 쉼표_없음_e02() {
    let result = modify("{철수 이}");
    assert_eq!(result, "{E02}");
}

#[test]
fn 빈_단어_e03() {
    let result = modify("{, 이}");
    assert_eq!(result, "{E03}");
}

#[test]
fn 빈_접사_e04() {
    let result = modify("{철수, }");
    assert_eq!(result, "{E04}");
}

// 처리 에러 테스트

#[test]
fn 용언_미존재_e10() {
    // 어미("세요")는 맞지만 용언 사전에 없는 단어
    let result = modify("{없는용언다, 세요}");
    assert_eq!(result, "{E10}");
}

#[test]
fn 어미_미존재_e11() {
    // 단어("먹다")는 용언이지만 어미가 없음
    let result = modify("{먹다, 잘못된어미}");
    assert_eq!(result, "{E11}");
}

#[test]
fn 토시_미존재_e12() {
    // 단어도 용언이 아니고 토시(조사)도 아님
    let result = modify("{철수, 잘못된조사}");
    assert_eq!(result, "{E12}");
}

// 에러가 있어도 나머지는 정상 처리

#[test]
fn 부분_에러_처리() {
    let result = modify("{철수, 이} {, 을} {밥, 을} 먹었다.");
    assert_eq!(result, "철수가 {E03} 밥을 먹었다.");
}

// === 토시(조사) 조합 테스트 ===

#[test]
fn 토시_은는() {
    assert_eq!(modify("{철수, 은}"), "철수는");
    assert_eq!(modify("{밥, 은}"), "밥은");
}

#[test]
fn 토시_이가() {
    assert_eq!(modify("{철수, 이}"), "철수가");
    assert_eq!(modify("{밥, 이}"), "밥이");
}

#[test]
fn 토시_을를() {
    assert_eq!(modify("{철수, 을}"), "철수를");
    assert_eq!(modify("{밥, 을}"), "밥을");
}

#[test]
fn 토시_과와() {
    assert_eq!(modify("{철수, 과}"), "철수와");
    assert_eq!(modify("{밥, 과}"), "밥과");
}

#[test]
fn 토시_으로() {
    assert_eq!(modify("{집, 으로}"), "집으로");
    assert_eq!(modify("{바다, 으로}"), "바다로");
}

// === 용언 활용 테스트 ===

#[test]
fn 용언_과거형() {
    assert_eq!(modify("{먹다, 었습니다}"), "먹었습니다");
}

#[test]
fn 용언_존칭() {
    assert_eq!(modify("{쉬다, 세요}"), "쉬세요");
    assert_eq!(modify("{먹다, 세요}"), "먹으세요");
}

#[test]
fn 용언_는데() {
    assert_eq!(modify("{먹다, 는데}"), "먹는데");
}

// === 엣지 케이스 ===

#[test]
fn 빈_문자열() {
    assert_eq!(modify(""), "");
}

#[test]
fn 중괄호만() {
    assert_eq!(modify("{}"), "{E02}");
}

#[test]
fn 패턴_연속() {
    let result = modify("{철수, 이}{밥, 을} 먹었다.");
    assert_eq!(result, "철수가밥을 먹었다.");
}

#[test]
fn 패턴_없이_중괄호_닫힘만() {
    assert_eq!(modify("abc} def"), "abc} def");
}

#[test]
fn 공백만_있는_입력() {
    assert_eq!(modify("   "), "   ");
}

#[test]
fn 여러_에러_연속() {
    let result = modify("{} {철수 이} {, 을}");
    assert_eq!(result, "{E02} {E02} {E03}");
}

#[test]
fn 닫는_중괄호_없음_뒤에_텍스트() {
    let result = modify("앞 {철수, 이");
    assert_eq!(result, "앞 {E01}");
}

#[test]
fn 패턴_사이_텍스트_유지() {
    let result = modify("오늘 {철수, 이} 학교에서 {밥, 을} 먹었다.");
    assert_eq!(result, "오늘 철수가 학교에서 밥을 먹었다.");
}

// === 시나리오 테스트 ===

#[test]
fn 게임_전투_로그() {
    let result = modify("{플레이어, 이} {몬스터, 을} {공격하다, 었습니다}.");
    assert_eq!(result, "플레이어가 몬스터를 공격했습니다.");
}

#[test]
fn 게임_아이템_획득() {
    let result = modify("{플레이어, 이} {포션, 을} 획득했습니다.");
    assert_eq!(result, "플레이어가 포션을 획득했습니다.");
}

#[test]
fn npc_대사() {
    let result = modify("여기서 {쉬다, 세요}. {걱정, 은} 마세요.");
    assert_eq!(result, "여기서 쉬세요. 걱정은 마세요.");
}

#[test]
fn 시스템_메시지() {
    let result = modify("{사용자, 이} 방에 입장했습니다.");
    assert_eq!(result, "사용자가 방에 입장했습니다.");
}

#[test]
fn 긴_문장_복합_처리() {
    let result = modify(
        "{영희, 이} {철수, 과} 함께 {도서관, 으로} {가다, 었습니다}."
    );
    assert_eq!(result, "영희가 철수와 함께 도서관으로 갔습니다.");
}

#[test]
fn 게임_퀘스트_완료() {
    let result = modify(
        "{용사, 이} {마왕, 을} {공격하다, 었습니다}. \
         {왕국, 은} 다시 {평화, 을} 되찾았습니다."
    );
    assert_eq!(result, "용사가 마왕을 공격했습니다. 왕국은 다시 평화를 되찾았습니다.");
}

#[test]
fn 채팅_시스템_메시지() {
    let result = modify(
        "{관리자, 이} {공지사항, 을} 등록했습니다. \
         {사용자, 은} {채팅방, 으로} {돌아가다, 세요}."
    );
    assert_eq!(result, "관리자가 공지사항을 등록했습니다. 사용자는 채팅방으로 돌아가세요.");
}
