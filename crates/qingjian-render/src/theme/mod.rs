//! 主题：字体、颜色、间距。所有可视参数都在这里，单位是点；将来从 TOML 读。
//!
//! 视觉层级（产品决定）：候选词最深，译文稍浅，词性最浅，序号弱化。数值对齐 macOS 壳的 AppKit 实现。

mod font_spec;
mod palette;
mod shape;
mod style;

pub use font_spec::FontSpec;
pub use palette::Palette;
pub use shape::{Decoration, HighlightShape};
pub use style::Style;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// 候选词字体。
    pub text_font: FontSpec,

    /// 译文与词性字体。
    pub annotation_font: FontSpec,

    /// 序号字体。
    pub index_font: FontSpec,

    /// 配色。
    pub colors: Palette,

    /// 窗口内边距。
    pub padding: f32,

    /// 行内上下留白。
    pub row_padding: f32,

    /// 序号与候选词、候选词与译文之间的间距。
    pub column_gap: f32,

    /// 窗口与高亮条的圆角。
    pub corner_radius: f32,

    /// 最多显示几行。
    pub max_rows: usize,

    /// 文字抗锯齿覆盖率的 gamma：小于 1 笔画显粗。CoreText 对文字有一层类似的加深，深色背景上尤其明显，
    /// 线性混合出来的字会偏细；这个值按真机截图并排调。
    pub text_gamma: f32,

    /// 高亮条的形状：圆角矩形 / 胶囊（两端全圆）。
    pub highlight_shape: HighlightShape,

    /// 背景角落的小纹样，低透明度画在背景上，候选内容会盖在它上面。
    pub decoration: Decoration,
}

impl Theme {
    /// 浅色，对齐 macOS 系统外观。
    pub fn light() -> Self {
        Self::with_palette(Palette::light(), 0.85)
    }

    /// 深色，对齐 macOS 系统外观。
    pub fn dark() -> Self {
        Self::with_palette(Palette::dark(), 0.75)
    }

    /// 按配色风格与深浅外观取一套。
    pub fn for_style(style: Style, dark: bool) -> Self {
        let (colors, gamma) = match (style, dark) {
            (Style::Default, false) => (Palette::light(), 0.85),
            (Style::Default, true) => (Palette::dark(), 0.75),
            (Style::Ink, false) => (Palette::ink_light(), 0.82),
            (Style::Ink, true) => (Palette::ink_dark(), 0.78),
            (Style::Celadon, false) => (Palette::celadon_light(), 0.83),
            (Style::Celadon, true) => (Palette::celadon_dark(), 0.77),
            (Style::Amber, false) => (Palette::amber_light(), 0.84),
            (Style::Amber, true) => (Palette::amber_dark(), 0.78),
            (Style::Nightflight, false) => (Palette::nightflight_light(), 0.84),
            (Style::Nightflight, true) => (Palette::nightflight_dark(), 0.76),
            (Style::Sakura, false) => (Palette::sakura_light(), 0.83),
            (Style::Sakura, true) => (Palette::sakura_dark(), 0.77),
        };
        let mut theme = Self::with_palette(colors, gamma);
        match style {
            Style::Ink => {
                // 纸感：圆角小一号硬朗，高亮胶囊，右下一枚朱印
                theme.corner_radius = 6.0;
                theme.highlight_shape = HighlightShape::Pill;
                theme.decoration = Decoration::Seal;
            }
            Style::Celadon => {
                // 瓷感：方正 + 双圈纹
                theme.corner_radius = 6.0;
                theme.decoration = Decoration::Ring;
            }
            Style::Amber => {
                // 暖感：圆润 + 角落弧线
                theme.corner_radius = 12.0;
                theme.decoration = Decoration::Arcs;
            }
            Style::Nightflight => {
                // 夜感：胶囊高亮 + 星点
                theme.corner_radius = 10.0;
                theme.highlight_shape = HighlightShape::Pill;
                theme.decoration = Decoration::Stars;
            }
            Style::Sakura => {
                // 花感：最圆润 + 花瓣
                theme.corner_radius = 12.0;
                theme.highlight_shape = HighlightShape::Pill;
                theme.decoration = Decoration::Petals;
            }
            Style::Default => {}
        }
        theme
    }

    fn with_palette(colors: Palette, text_gamma: f32) -> Self {
        Self {
            // 行高取 AppKit 系统字体在这几个字号下 NSAttributedString.size() 的高度
            text_font: FontSpec::new(16.0, 19.0),
            annotation_font: FontSpec::new(12.0, 15.0),
            index_font: FontSpec::new(11.0, 14.0),
            colors,
            padding: 8.0,
            row_padding: 4.0,
            column_gap: 8.0,
            corner_radius: 8.0,
            max_rows: 9,
            text_gamma,
            highlight_shape: HighlightShape::Rounded,
            decoration: Decoration::None,
        }
    }
}
