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
        }
    }

    /// 毛玻璃浅色：半透明白叠在系统材质上，文字加深一档抵住透底。
    pub const fn frosted_light() -> Self {
        Self {
            text: Color::gray(0, 235),
            gloss: Color::gray(0, 150),
            pos: Color::gray(0, 90),
            fresh: Color::rgb(220, 120, 20),
            index: Color::gray(0, 90),
            cloud: Color::rgb(0, 150, 165),
            background: Color::rgba(255, 255, 255, 145),
            highlight: Color::rgba(0, 110, 235, 65),
        }
    }

    /// 毛玻璃深色：半透明炭叠在系统材质上。
    pub const fn frosted_dark() -> Self {
        Self {
            text: Color::gray(255, 230),
            gloss: Color::gray(255, 155),
            pos: Color::gray(255, 80),
            fresh: Color::rgb(255, 160, 70),
            index: Color::gray(255, 80),
            cloud: Color::rgb(70, 195, 210),
            background: Color::rgba(24, 24, 27, 160),
            highlight: Color::rgba(95, 150, 255, 75),
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
        }
    }
}
