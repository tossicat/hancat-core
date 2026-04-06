# hancat-core

[tossicat-core](https://github.com/tossicat/tossicat-core)(토시(조사))와 [yongcat](https://github.com/tossicat/yongcat)(용언 활용)을 통합하는 한국어 텍스트 처리 라이브러리입니다.

`{단어, 접사}` 형태의 템플릿으로 토시(조사)와 용언 활용을 자동으로 처리합니다.

## 사용 예

```rust
use hancat_core::modify;

// 토시(조사) + 용언 통합 처리
let result = modify("{철수, 이} {밥, 을} {먹다, 었습니다}.");
assert_eq!(result, "철수가 밥을 먹었습니다.");

// 게임 전투 로그
let result = modify("{플레이어, 이} {몬스터, 을} {공격하다, 었습니다}.");
assert_eq!(result, "플레이어가 몬스터를 공격했습니다.");

// NPC 대사
let result = modify("여기서 {쉬다, 세요}.");
assert_eq!(result, "여기서 쉬세요.");
```

## 동작 방식

`{단어, 접사}` 패턴을 만나면 접사의 종류에 따라 자동으로 처리를 분기합니다.

- 접사가 어미이고 단어가 용언 사전에 있으면 yongcat으로 활용형을 생성합니다.
- 그 외에는 tossicat-core로 토시(조사)를 처리합니다.
- 처리 실패 시 에러 코드(`{E01}`~`{E12}`)를 해당 위치에 삽입하고, 나머지는 정상 처리합니다.

## 에러 코드

처리 중 문제가 발생하면 해당 위치에 에러 코드가 삽입됩니다.

| 코드 | 구분 | 설명 | 예시 입력 | 출력 |
|------|------|------|-----------|------|
| `{E01}` | 파싱 | 닫는 중괄호 없음 | `{철수, 이` | `{E01}` |
| `{E02}` | 파싱 | 쉼표 없음 | `{철수 이}` | `{E02}` |
| `{E03}` | 파싱 | 빈 단어 | `{, 이}` | `{E03}` |
| `{E04}` | 파싱 | 빈 접사 | `{철수, }` | `{E04}` |
| `{E10}` | 처리 | 용언 미존재 | `{없는용언다, 세요}` | `{E10}` |
| `{E11}` | 처리 | 어미 미존재 | `{먹다, 잘못된어미}` | `{E11}` |
| `{E12}` | 처리 | 토시(조사) 미존재 | `{철수, 잘못된조사}` | `{E12}` |

## Feature Flags

필요에 따라 포함할 기능을 선택할 수 있습니다.

```toml
# 전체 기능 (기본값)
hancat-core = "0.8"

# 토시(조사)만
hancat-core = { version = "0.8", features = ["tossi"], default-features = false }

# 용언만
hancat-core = { version = "0.8", features = ["yongeon"], default-features = false }

# 용언 A등급만 (230개, 용량 최소화)
hancat-core = { version = "0.8", features = ["grade-a"], default-features = false }

# 용언 A+B등급 (863개)
hancat-core = { version = "0.8", features = ["grade-b"], default-features = false }
```

| Feature | 설명 | 용언 수 |
|---------|------|---------|
| `tossi` | 토시(조사) 처리 (tossicat-core) | - |
| `yongeon` | 용언 활용 (yongcat, 전체) | 1,721개 |
| `grade-a` | 용언 A등급만 | 230개 |
| `grade-b` | 용언 A+B등급 | 863개 |

## 사용자 용언 추가

기본 사전에 없는 용언은 CSV 파일로 추가할 수 있습니다. yongcat의 `data/` 폴더에 CSV 파일을 작성하고 빌드하면 `modify()`에서 자동으로 사용됩니다.

```csv
base_form,dict_id,eogan,pos,conjugation,usage,grade
쓰러뜨리다,,쓰러뜨리,동사,규,,
부수다,,부수,동사,규,,
```

```bash
# 검증
cargo run --bin import

# 빌드
cargo build
```

빌드 후에는 추가한 용언이 바로 사용 가능합니다:

```rust
let result = modify("{용사, 이} {마왕, 을} {쓰러뜨리다, 었습니다}.");
assert_eq!(result, "용사가 마왕을 쓰러뜨렸습니다.");
```

CSV 형식, 활용 유형 코드, 검증 방법 등 자세한 내용은 [yongcat USER_DATA.md](https://docs.rs/crate/yongcat/0.8.2/source/USER_DATA.md)를 참고하세요.

> **참고:** 토시(조사)는 사용자가 직접 추가할 수 없습니다. 토시(조사) 추가가 필요하면 [tossicat-core 이슈](https://github.com/tossicat/tossicat-core/issues)에 요청해 주세요.

## 의존성

- [tossicat-core](https://github.com/tossicat/tossicat-core) - 한국어 토시(조사) 처리 (205개 토시(조사))
- [yongcat](https://github.com/tossicat/yongcat) - 한국어 용언 활용 (1,721개 용언, 42개 어미)

## 라이선스

MIT
