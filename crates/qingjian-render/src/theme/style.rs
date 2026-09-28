//! 候选窗口的配色风格：与平台无关，平台配置枚举各自映射到这边。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// 系统观感：白底 / 深灰底，不透明。
    Default,

    /// 墨韵：浅色宣纸底 + 朱砂高亮，深色换炭底暖字，不透明。
    Ink,

    /// 青瓷：青瓷白绿底 + 深青字，玉色高亮。
    Celadon,

    /// 琥珀：米黄暖底 + 深褐字，琥珀高亮。
    Amber,

    /// 夜航：冷蓝白底 + 藏青字，深色换海军蓝。
    Nightflight,

    /// 樱花：樱粉底 + 紫褐字，玫瑰高亮。
    Sakura,
}
