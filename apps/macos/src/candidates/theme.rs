//! 候选窗口主题：字体、颜色、间距。所有可视参数集中在这里。
//!
//! 视觉层级（产品决定）：候选词最深，译文稍浅，词性最浅，序号弱化。
//! 颜色随 `[general] candidate_style` 与深浅外观变化，见 [`Theme::apply_style`]。

use objc2::rc::Retained;
use objc2_app_kit::{NSColor, NSFont};
use qingjian_platform::CandidateStyle;

pub struct Theme {
    /// 候选词字体。
    pub text_font: Retained<NSFont>,

    /// 译文与词性字体。
    pub annotation_font: Retained<NSFont>,

    /// 序号字体。
    pub index_font: Retained<NSFont>,

    /// 候选词颜色。
    pub text_color: Retained<NSColor>,

    /// 译文颜色。
    pub gloss_color: Retained<NSColor>,

    /// 词性颜色，比译文更浅。
    pub pos_color: Retained<NSColor>,

    /// 生词译文的颜色：比普通译文醒目，提醒「这个词你还没见过几次」，看熟了就回到译文色。
    pub fresh_color: Retained<NSColor>,

    /// 序号颜色。
    pub index_color: Retained<NSColor>,

    /// 云联想的云朵与文字颜色：比译文醒目一点，但仍不抢候选词。
    pub cloud_color: Retained<NSColor>,

    /// 窗口背景。毛玻璃下带 alpha，透出窗口底层的系统材质。
    pub background: Retained<NSColor>,

    /// 当前候选的高亮底色。
    pub highlight: Retained<NSColor>,

    /// 窗口内边距。
    pub padding: f64,

    /// 行内上下留白。
    pub row_padding: f64,

    /// 序号与候选词、候选词与译文之间的间距。
    pub column_gap: f64,

    /// 窗口与高亮条的圆角。
    pub corner_radius: f64,

    /// 窗口描边：毛玻璃下勾出玻璃轮廓，其余风格与背景同色。
    pub border_color: Retained<NSColor>,

    /// 最多显示几行。
    pub max_rows: usize,

    /// 是否在窗口底层垫毛玻璃材质（`frosted` 风格）；窗口侧读它装 / 卸 NSVisualEffectView。
    pub vibrancy: bool,
}

impl Theme {
    /// 系统默认外观。只能在主线程调用（NSFont / NSColor 不跨线程）。
    pub fn system_default() -> Self {
        Self {
            text_font: NSFont::systemFontOfSize(16.0),
            annotation_font: NSFont::systemFontOfSize(12.0),
            index_font: NSFont::systemFontOfSize(11.0),
            text_color: NSColor::labelColor(),
            gloss_color: NSColor::secondaryLabelColor(),
            pos_color: NSColor::tertiaryLabelColor(),
            fresh_color: NSColor::systemOrangeColor(),
            index_color: NSColor::tertiaryLabelColor(),
            cloud_color: NSColor::systemTealColor(),
            background: NSColor::windowBackgroundColor(),
            highlight: NSColor::colorWithSRGBRed_green_blue_alpha(0.0, 0.48, 1.0, 0.16),
            border_color: NSColor::windowBackgroundColor(),
            padding: 8.0,
            row_padding: 4.0,
            column_gap: 8.0,
            corner_radius: 8.0,
            max_rows: 9,
            vibrancy: false,
        }
    }

    /// 按配色风格与深浅外观改写颜色 / 圆角 / 毛玻璃开关；字体不动（用户可能另选了字族）。
    /// 数值对齐 qingjian-render 的同名调色板，两条渲染路径看起来一致。
    pub fn apply_style(&mut self, style: CandidateStyle, dark: bool) {
        *self = Self::system_default();
        match (style, dark) {
            (CandidateStyle::Default, _) => {}
            (CandidateStyle::Frosted, _) => {
                // 深色 HUD 玻璃，两种外观同一套：材质本身就深，浅色下也一眼可见
                self.text_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.92, 0.92, 0.92, 0.94);
                self.gloss_color = NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 1.0, 1.0, 0.65);
                self.pos_color = NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 1.0, 1.0, 0.37);
                self.index_color = self.pos_color.clone();
                self.fresh_color =
                    NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 0.67, 0.31, 1.0);
                self.cloud_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.78, 0.84, 1.0);
                self.background =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.06, 0.06, 0.08, 0.35);
                self.highlight =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.37, 0.59, 1.0, 0.33);
                self.border_color = NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 1.0, 1.0, 0.22);
                self.corner_radius = 10.0;
                self.vibrancy = true;
            }
            (CandidateStyle::Ink, false) => {
                self.text_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.15, 0.15, 0.16, 1.0);
                self.gloss_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.44, 0.42, 0.39, 1.0);
                self.pos_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.59, 0.57, 0.54, 1.0);
                self.index_color = self.pos_color.clone();
                self.fresh_color =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.70, 0.23, 0.17, 1.0);
                self.cloud_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.0, 0.55, 0.61, 1.0);
                self.background =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.98, 0.97, 0.95, 1.0);
                self.border_color =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.89, 0.87, 0.82, 1.0);
                self.highlight =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.70, 0.23, 0.17, 0.20);
                self.corner_radius = 6.0;
            }
            (CandidateStyle::Ink, true) => {
                self.text_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.91, 0.90, 0.88, 1.0);
                self.gloss_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.63, 0.61, 0.57, 1.0);
                self.pos_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.43, 0.42, 0.39, 1.0);
                self.index_color = self.pos_color.clone();
                self.fresh_color =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.88, 0.41, 0.31, 1.0);
                self.cloud_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 0.78, 1.0);
                self.background =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.10, 0.10, 0.11, 1.0);
                self.border_color =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.17, 0.17, 0.18, 1.0);
                self.highlight =
                    NSColor::colorWithSRGBRed_green_blue_alpha(0.78, 0.36, 0.27, 0.31);
                self.corner_radius = 6.0;
            }
        }
    }
}
