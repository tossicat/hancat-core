# hancat-core

[tossicat-core](https://github.com/tossicat/tossicat-core)(조사)와 [yongcat](https://github.com/tossicat/yongcat)(용언 활용)을 통합하는 한국어 텍스트 처리 라이브러리입니다.

`{단어, 접사}` 형태의 템플릿으로 조사와 용언 활용을 자동으로 처리합니다.

## 사용 예

```rust
use hancat_core::modify_sentence;

// 조사 + 용언 통합 처리
let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.").unwrap();
assert_eq!(result, "철수가 밥을 먹었습니다.");

// 게임 전투 로그
let result = modify_sentence("{플레이어, 이} {몬스터, 을} {공격하다, 었습니다}.").unwrap();
assert_eq!(result, "플레이어가 몬스터를 공격했습니다.");

// NPC 대사
let result = modify_sentence("여기서 {쉬다, 세요}.").unwrap();
assert_eq!(result, "여기서 쉬세요.");
```

## 동작 방식

`{단어, 접사}` 패턴을 만나면 단어의 형태에 따라 자동으로 처리를 분기합니다.

- 단어가 `다`로 끝나면 용언으로 판단하여 yongcat으로 활용형을 생성합니다.
- 그 외에는 명사로 판단하여 tossicat-core로 조사를 처리합니다.

## Feature Flags

필요에 따라 포함할 기능을 선택할 수 있습니다.

```toml
# 전체 기능 (기본값)
hancat-core = "0.1"

# 조사만
hancat-core = { version = "0.1", features = ["tossi"], default-features = false }

# 용언만
hancat-core = { version = "0.1", features = ["yongeon"], default-features = false }

# 용언 A등급만 (230개, 용량 최소화)
hancat-core = { version = "0.1", features = ["grade-a"], default-features = false }

# 용언 A+B등급 (863개)
hancat-core = { version = "0.1", features = ["grade-b"], default-features = false }
```

| Feature | 설명 | 용언 수 |
|---------|------|---------|
| `tossi` | 조사 처리 (tossicat-core) | - |
| `yongeon` | 용언 활용 (yongcat, 전체) | 1,721개 |
| `grade-a` | 용언 A등급만 | 230개 |
| `grade-b` | 용언 A+B등급 | 863개 |

## 의존성

- [tossicat-core](https://github.com/tossicat/tossicat-core) - 한국어 조사 처리 (138개 조사)
- [yongcat](https://github.com/tossicat/yongcat) - 한국어 용언 활용 (1,721개 용언, 42개 어미)

## 라이선스

MIT
