//! 候选窗口的配色风格。

use serde::{Deserialize, Serialize};

/// 候选窗口配色：`default` 系统观感、`ink` 墨韵、`celadon` 青瓷、`amber` 琥珀、
/// `nightflight` 夜航、`sakura` 樱花。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStyle {
    /// 系统观感（缺省）：白底 / 深灰底，不透明。
    Default,

    /// 墨韵：宣纸底 + 朱砂高亮，圆角略小。
    Ink,

    /// 青瓷：青瓷白绿底 + 玉色高亮。
    Celadon,

    /// 琥珀：米黄暖底 + 琥珀高亮。
    Amber,

    /// 夜航：冷蓝白底 + 钴蓝高亮，深色换海军蓝。
    Nightflight,

    /// 樱花：樱粉底 + 玫瑰高亮。
    Sakura,
}

impl Default for CandidateStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl CandidateStyle {
    /// 全部风格的名字与菜单标签，顺序即菜单顺序。
    pub const NAMES: [(&'static str, &'static str); 6] = [
        ("default", "系统观感"),
        ("ink", "墨韵"),
        ("celadon", "青瓷"),
        ("amber", "琥珀"),
        ("nightflight", "夜航"),
        ("sakura", "樱花"),
    ];

    /// 写配置文件用的键（serde 键一致）。
    pub fn key(self) -> &'static str {
        Self::NAMES[self.to_index()].0
    }

    fn to_index(self) -> usize {
        match self {
            Self::Default => 0,
            Self::Ink => 1,
            Self::Celadon => 2,
            Self::Amber => 3,
            Self::Nightflight => 4,
            Self::Sakura => 5,
        }
    }

    /// 从配置字符串解析；不认识的写法按缺省并警告。
    pub fn parse(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "ink" => Self::Ink,
            "celadon" => Self::Celadon,
            "amber" => Self::Amber,
            "nightflight" => Self::Nightflight,
            "sakura" => Self::Sakura,
            other => {
                tracing::warn!(value = %other, "candidate_style 取值不认识，按 default 处理");
                Self::Default
            }
        }
    }
}
