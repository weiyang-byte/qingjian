//! 一套配色。缺省两套取自 macOS 系统语义色在 sRGB 下的实测值（label 0.847、secondaryLabel 0.498…）。

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    /// 候选词。
    pub text: Color,

    /// 译文。
    pub gloss: Color,

    /// 词性，比译文更浅。
    pub pos: Color,

    /// 生词译文：比普通译文醒目，看熟了就回到译文色。
    pub fresh: Color,

    /// 序号。
    pub index: Color,

    /// 云联想的云朵与文字：比译文醒目一点，但不抢候选词。
    pub cloud: Color,

    /// 窗口背景。
    pub background: Color,

    /// 当前候选的高亮底色。
    pub highlight: Color,

    /// 窗口描边：与背景不同色时（毛玻璃），背景外面先铺一圈它再铺内底，玻璃有轮廓。
    pub border: Color,
}

impl Palette {
    pub const fn light() -> Self {
        Self {
            text: Color::gray(0, 216),
            gloss: Color::gray(0, 127),
            pos: Color::gray(0, 66),
            fresh: Color::rgb(255, 141, 40),
            index: Color::gray(0, 66),
            cloud: Color::rgb(0, 195, 208),
            background: Color::rgb(255, 255, 255),
            highlight: Color::rgba(176, 206, 125, 127),
            border: Color::rgb(255, 255, 255),
        }
    }

    pub const fn dark() -> Self {
        Self {
            text: Color::gray(255, 216),
            gloss: Color::gray(255, 140),
            pos: Color::gray(255, 63),
            fresh: Color::rgb(255, 146, 48),
            index: Color::gray(255, 63),
            cloud: Color::rgb(0, 210, 224),
            background: Color::rgb(30, 30, 30),
            highlight: Color::rgba(36, 76, 36, 255),
            border: Color::rgb(30, 30, 30),
        }
    }

    /// 毛玻璃（浅色外观）：深色 HUD 玻璃，两种外观同一套——材质本身就深，浅色下也一眼可见。
    pub const fn frosted_light() -> Self {
        Self::frosted_glass()
    }

    /// 毛玻璃（深色外观）：同上。
    pub const fn frosted_dark() -> Self {
        Self::frosted_glass()
    }

    /// 毛玻璃本体：深色 HUD 材质上的半透明炭面，亮字、蓝高亮、白描边。
    pub const fn frosted_glass() -> Self {
        Self {
            text: Color::gray(235, 240),
            gloss: Color::gray(255, 165),
            pos: Color::gray(255, 95),
            fresh: Color::rgb(255, 170, 80),
            index: Color::gray(255, 95),
            cloud: Color::rgb(90, 200, 215),
            background: Color::rgba(16, 16, 20, 90),
            highlight: Color::rgba(95, 150, 255, 85),
            border: Color::rgba(255, 255, 255, 55),
        }
    }

    /// 墨韵浅色：宣纸底、浓墨字、朱砂高亮。
    pub const fn ink_light() -> Self {
        Self {
            text: Color::rgb(38, 38, 40),
            gloss: Color::rgb(112, 108, 100),
            pos: Color::rgb(150, 146, 138),
            fresh: Color::rgb(178, 58, 44),
            index: Color::rgb(150, 146, 138),
            cloud: Color::rgb(0, 140, 155),
            background: Color::rgb(250, 248, 242),
            highlight: Color::rgba(178, 58, 44, 52),
            border: Color::rgb(226, 222, 210),
        }
    }

    /// 墨韵深色：炭底暖字，朱砂稍亮。
    pub const fn ink_dark() -> Self {
        Self {
            text: Color::rgb(233, 230, 224),
            gloss: Color::rgb(160, 155, 145),
            pos: Color::rgb(110, 106, 100),
            fresh: Color::rgb(225, 105, 80),
            index: Color::rgb(110, 106, 100),
            cloud: Color::rgb(90, 190, 200),
            background: Color::rgb(25, 25, 27),
            highlight: Color::rgba(200, 92, 70, 80),
            border: Color::rgb(44, 44, 47),
        }
    }
}
