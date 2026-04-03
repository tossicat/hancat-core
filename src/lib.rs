//! # hancat-core
//!
//! tossicat-core(조사)와 yongcat(용언 활용)을 통합하는 한국어 텍스트 처리 라이브러리.
//!
//! `{단어, 접사}` 형태의 템플릿을 파싱하여, 조사와 용언 활용을 자동으로 처리합니다.
//!
//! ## 사용 예
//!
//! ```rust
//! use hancat_core::modify_sentence;
//!
//! let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.");
//! assert_eq!(result, "철수가 밥을 먹었습니다.");
//! ```

/// 단어와 접사 쌍을 처리하여 변환된 문자열을 반환합니다.
///
/// 접사가 어미이고 단어가 용언이면 용언 활용, 아니면 조사로 처리합니다.
/// 실패 시 에러 코드를 반환합니다.
fn process_pair(word: &str, suffix: &str) -> Result<String, &'static str> {
    // 접사가 어미이고 단어가 용언 사전에 있으면 용언 활용 처리
    #[cfg(feature = "yongeon")]
    if let Some(eomi) = yongcat::find_eomi_exact(suffix) {
        if let Some(yongeon) = yongcat::lookup_all(word).into_iter().next() {
            return Ok(yongcat::conjugate(yongeon, eomi));
        }
    }

    // 그 외에는 조사로 처리
    #[cfg(feature = "tossi")]
    {
        tossicat::postfix(word, suffix).map_err(|_| "E11")
    }
    #[cfg(not(feature = "tossi"))]
    {
        Ok(format!("{}{}", word, suffix))
    }
}

/// 문장 내의 모든 `{단어, 접사}` 패턴을 처리하여 변환된 문장을 반환합니다.
///
/// - 접사가 어미이면 용언 활용 처리 (yongeon feature 필요)
/// - 그 외에는 조사 처리 (tossi feature 필요)
/// - 처리 실패 시 에러 코드(`{E01}`~`{E11}`)를 해당 위치에 삽입합니다.
///
/// # 예제
///
/// ```rust
/// use hancat_core::modify_sentence;
///
/// // 조사 + 용언 통합 처리
/// let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.");
/// assert_eq!(result, "철수가 밥을 먹었습니다.");
///
/// // 조사만 처리
/// let result = modify_sentence("{철수, 이} 왔다.");
/// assert_eq!(result, "철수가 왔다.");
///
/// // 용언만 처리
/// let result = modify_sentence("여기서 {쉬다, 세요}.");
/// assert_eq!(result, "여기서 쉬세요.");
///
/// // 파싱 에러 시 에러 코드 반환
/// let result = modify_sentence("{철수 이}");
/// assert_eq!(result, "{E02}");
/// ```
pub fn modify_sentence(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.char_indices();
    let mut last_end = 0;

    while let Some((start, ch)) = chars.next() {
        if ch == '{' {
            // 닫는 중괄호 찾기
            let mut end = None;
            for (i, c) in chars.by_ref() {
                if c == '}' {
                    end = Some(i);
                    break;
                }
            }

            let Some(end) = end else {
                // E01: 닫는 중괄호 없음
                result.push_str(&input[last_end..start]);
                result.push_str("{E01}");
                last_end = input.len();
                break;
            };

            result.push_str(&input[last_end..start]);
            let inner = &input[start + '{'.len_utf8()..end];

            let Some(comma_pos) = inner.find(',') else {
                // E02: 쉼표 없음
                result.push_str("{E02}");
                last_end = end + '}'.len_utf8();
                continue;
            };

            let word = inner[..comma_pos].trim();
            let suffix = inner[comma_pos + 1..].trim();

            if word.is_empty() {
                // E03: 빈 단어
                result.push_str("{E03}");
            } else if suffix.is_empty() {
                // E04: 빈 접사
                result.push_str("{E04}");
            } else {
                match process_pair(word, suffix) {
                    Ok(processed) => result.push_str(&processed),
                    Err(code) => {
                        result.push('{');
                        result.push_str(code);
                        result.push('}');
                    }
                }
            }

            last_end = end + '}'.len_utf8();
        }
    }

    // 남은 텍스트 추가
    result.push_str(&input[last_end..]);
    result
}
