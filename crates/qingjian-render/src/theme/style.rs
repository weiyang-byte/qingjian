//! 候选窗口的配色风格：与平台无关，平台配置枚举各自映射到这边。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// 系统观感：白底 / 深灰底，不透明。
    Default,

    /// 墨韵：浅色宣纸底 + 朱砂高亮，深色换炭底暖字，不透明。
    Ink,
}
