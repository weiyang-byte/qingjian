//! 候选窗口的配色风格。

use serde::{Deserialize, Serialize};

/// 候选窗口配色：`default` 系统观感、`ink` 墨韵（浅色宣纸底 + 朱砂高亮，深色炭底暖字）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStyle {
    /// 系统观感（缺省）：白底 / 深灰底，不透明。
    Default,

    /// 墨韵：纸墨配色，圆角略小。
    Ink,
}

impl Default for CandidateStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl CandidateStyle {
    /// 全部风格的名字与菜单标签，顺序即子菜单顺序。
    pub const NAMES: [(&'static str, &'static str); 2] =
        [("default", "系统观感"), ("ink", "墨韵")];

    /// 写配置文件用的键（serde 键一致）。
    pub fn key(self) -> &'static str {
        Self::NAMES[self.to_index()].0
    }

    fn to_index(self) -> usize {
        match self {
            Self::Default => 0,
            Self::Ink => 1,
        }
    }

    /// 从配置字符串解析；不认识的写法按缺省并警告。
    pub fn parse(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "ink" => Self::Ink,
            other => {
                tracing::warn!(value = %other, "candidate_style 取值不认识，按 default 处理");
                Self::Default
            }
        }
    }
}
