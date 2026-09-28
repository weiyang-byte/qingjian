//! 高亮形状与背景纹样：候选窗的「性格」参数，主题各配一套。

/// 高亮条的形状。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightShape {
    /// 圆角矩形，圆角是窗口圆角的一半。
    Rounded,

    /// 胶囊：两端全圆，圆角取行高一半。
    Pill,
}

/// 背景角落的小纹样：低透明度画在背景上，候选内容盖在它上面。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoration {
    /// 无。
    None,

    /// 朱印：右下一枚红色小方章（墨韵）。
    Seal,

    /// 双圈：右下两圈同心细环（青瓷）。
    Ring,

    /// 弧线：右下角三道同心弧（琥珀）。
    Arcs,

    /// 星点：右上角几粒小星（夜航）。
    Stars,

    /// 花瓣：右上角三片柔粉花瓣（樱花）。
    Petals,
}
