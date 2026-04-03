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
//! let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.").unwrap();
//! assert_eq!(result, "철수가 밥을 먹었습니다.");
//! ```

use std::fmt;

/// 에러 타입
#[derive(Debug)]
pub enum Error {
    /// 템플릿 파싱 에러
    Parse(String),
    /// 조사 처리 에러
    #[cfg(feature = "tossi")]
    Tossi(String),
    /// 용언 활용 에러
    #[cfg(feature = "yongeon")]
    Yongeon(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "파싱 에러: {}", msg),
            #[cfg(feature = "tossi")]
            Error::Tossi(msg) => write!(f, "조사 에러: {}", msg),
            #[cfg(feature = "yongeon")]
            Error::Yongeon(msg) => write!(f, "용언 에러: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// `{단어, 접사}` 쌍을 나타내는 구조체
#[derive(Debug)]
struct Pair {
    /// 원본 문자열 (중괄호 포함, 예: "{철수, 이}")
    original: String,
    /// 단어 부분 (예: "철수")
    word: String,
    /// 접사 부분 (예: "이")
    suffix: String,
}

/// 입력 문자열에서 `{단어, 접사}` 패턴을 모두 추출합니다.
fn parse_pairs(input: &str) -> Result<Vec<Pair>, Error> {
    let mut pairs = Vec::new();
    let mut chars = input.char_indices();

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
            let end = end.ok_or_else(|| {
                Error::Parse("닫는 중괄호 '}'를 찾을 수 없습니다".to_string())
            })?;

            let inner = &input[start + '{'.len_utf8()..end];

            // 쉼표로 분리
            let comma_pos = inner.find(',').ok_or_else(|| {
                Error::Parse(format!("쉼표를 찾을 수 없습니다: {{{}}}", inner))
            })?;

            let word = inner[..comma_pos].trim().to_string();
            let suffix = inner[comma_pos + 1..].trim().to_string();

            if word.is_empty() || suffix.is_empty() {
                return Err(Error::Parse(format!(
                    "단어 또는 접사가 비어 있습니다: {{{}}}",
                    inner
                )));
            }

            pairs.push(Pair {
                original: input[start..=end].to_string(),
                word,
                suffix,
            });
        }
    }

    Ok(pairs)
}

/// 단어와 접사 쌍을 처리하여 변환된 문자열을 반환합니다.
///
/// 접사가 어미이고 단어가 용언이면 용언 활용, 아니면 조사로 처리합니다.
fn process_pair(pair: &Pair) -> Result<String, Error> {
    // 접사가 어미이고 단어가 용언 사전에 있으면 용언 활용 처리
    #[cfg(feature = "yongeon")]
    if let Some(eomi) = yongcat::find_eomi_exact(&pair.suffix) {
        if !yongcat::lookup_all(&pair.word).is_empty() {
            return process_yongeon(&pair.word, eomi);
        }
    }

    // 그 외에는 조사로 처리
    #[cfg(feature = "tossi")]
    {
        process_tossi(&pair.word, &pair.suffix)
    }
    #[cfg(not(feature = "tossi"))]
    {
        Ok(format!("{}{}", &pair.word, &pair.suffix))
    }
}

/// tossicat을 사용하여 조사를 처리합니다.
#[cfg(feature = "tossi")]
fn process_tossi(word: &str, tossi: &str) -> Result<String, Error> {
    tossicat::postfix(word, tossi).map_err(|e| Error::Tossi(e.to_string()))
}

/// yongcat을 사용하여 용언 활용을 처리합니다.
#[cfg(feature = "yongeon")]
fn process_yongeon(word: &str, eomi: &yongcat::Eomi) -> Result<String, Error> {
    let yongeon = yongcat::lookup_all(word)
        .into_iter()
        .next()
        .ok_or_else(|| Error::Yongeon(format!("용언을 찾을 수 없습니다: {}", word)))?;
    Ok(yongcat::conjugate(yongeon, eomi))
}

/// 문장 내의 모든 `{단어, 접사}` 패턴을 처리하여 변환된 문장을 반환합니다.
///
/// - 접사가 어미이면 용언 활용 처리 (yongeon feature 필요)
/// - 그 외에는 조사 처리 (tossi feature 필요)
///
/// # 예제
///
/// ```rust
/// use hancat_core::modify_sentence;
///
/// // 조사 + 용언 통합 처리
/// let result = modify_sentence("{철수, 이} {밥, 을} {먹다, 었습니다}.").unwrap();
/// assert_eq!(result, "철수가 밥을 먹었습니다.");
///
/// // 조사만 처리
/// let result = modify_sentence("{철수, 이} 왔다.").unwrap();
/// assert_eq!(result, "철수가 왔다.");
///
/// // 용언만 처리
/// let result = modify_sentence("여기서 {쉬다, 세요}.").unwrap();
/// assert_eq!(result, "여기서 쉬세요.");
/// ```
pub fn modify_sentence(input: &str) -> Result<String, Error> {
    let pairs = parse_pairs(input)?;
    let mut result = input.to_string();

    for pair in &pairs {
        let replaced = process_pair(pair)?;
        result = result.replacen(&pair.original, &replaced, 1);
    }

    Ok(result)
}
