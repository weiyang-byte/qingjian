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

    /// 青瓷浅色：青瓷白绿底，深青字，玉色高亮。
    pub const fn celadon_light() -> Self {
        Self {
            text: Color::rgb(34, 64, 53),
            gloss: Color::rgb(96, 120, 110),
            pos: Color::rgb(140, 158, 150),
            fresh: Color::rgb(190, 70, 90),
            index: Color::rgb(140, 158, 150),
            cloud: Color::rgb(0, 140, 150),
            background: Color::rgb(238, 244, 240),
            highlight: Color::rgba(60, 140, 110, 70),
            border: Color::rgb(215, 224, 218),
        }
    }

    /// 青瓷深色：深青底，青白字。
    pub const fn celadon_dark() -> Self {
        Self {
            text: Color::rgb(225, 238, 230),
            gloss: Color::rgb(150, 175, 163),
            pos: Color::rgb(105, 125, 115),
            fresh: Color::rgb(255, 120, 100),
            index: Color::rgb(105, 125, 115),
            cloud: Color::rgb(80, 190, 200),
            background: Color::rgb(24, 34, 30),
            highlight: Color::rgba(70, 170, 130, 90),
            border: Color::rgb(45, 58, 52),
        }
    }

    /// 琥珀浅色：米黄暖底，深褐字，琥珀高亮。
    pub const fn amber_light() -> Self {
        Self {
            text: Color::rgb(74, 56, 38),
            gloss: Color::rgb(140, 116, 90),
            pos: Color::rgb(170, 148, 120),
            fresh: Color::rgb(220, 100, 30),
            index: Color::rgb(170, 148, 120),
            cloud: Color::rgb(200, 120, 40),
            background: Color::rgb(248, 242, 230),
            highlight: Color::rgba(230, 140, 40, 75),
            border: Color::rgb(232, 222, 204),
        }
    }

    /// 琥珀深色：暖褐底，暖白字。
    pub const fn amber_dark() -> Self {
        Self {
            text: Color::rgb(240, 225, 205),
            gloss: Color::rgb(180, 158, 130),
            pos: Color::rgb(130, 112, 90),
            fresh: Color::rgb(255, 150, 60),
            index: Color::rgb(130, 112, 90),
            cloud: Color::rgb(230, 160, 70),
            background: Color::rgb(34, 28, 22),
            highlight: Color::rgba(230, 150, 60, 90),
            border: Color::rgb(60, 50, 40),
        }
    }

    /// 夜航浅色：冷蓝白底，藏青字，钴蓝高亮。
    pub const fn nightflight_light() -> Self {
        Self {
            text: Color::rgb(28, 42, 66),
            gloss: Color::rgb(100, 115, 140),
            pos: Color::rgb(140, 152, 170),
            fresh: Color::rgb(255, 110, 90),
            index: Color::rgb(140, 152, 170),
            cloud: Color::rgb(60, 120, 220),
            background: Color::rgb(235, 240, 248),
            highlight: Color::rgba(50, 100, 220, 70),
            border: Color::rgb(212, 220, 232),
        }
    }

    /// 夜航深色：深海军蓝底，冰蓝字。
    pub const fn nightflight_dark() -> Self {
        Self {
            text: Color::rgb(214, 226, 246),
            gloss: Color::rgb(140, 158, 190),
            pos: Color::rgb(100, 112, 135),
            fresh: Color::rgb(255, 120, 100),
            index: Color::rgb(100, 112, 135),
            cloud: Color::rgb(110, 170, 255),
            background: Color::rgb(16, 22, 36),
            highlight: Color::rgba(90, 140, 255, 90),
            border: Color::rgb(48, 58, 80),
        }
    }

    /// 樱花浅色：樱粉底，紫褐字，玫瑰高亮。
    pub const fn sakura_light() -> Self {
        Self {
            text: Color::rgb(88, 44, 62),
            gloss: Color::rgb(150, 100, 120),
            pos: Color::rgb(180, 135, 152),
            fresh: Color::rgb(220, 60, 110),
            index: Color::rgb(180, 135, 152),
            cloud: Color::rgb(225, 100, 140),
            background: Color::rgb(250, 240, 243),
            highlight: Color::rgba(225, 120, 150, 80),
            border: Color::rgb(240, 220, 226),
        }
    }

    /// 樱花深色：深梅紫底，粉白字。
    pub const fn sakura_dark() -> Self {
        Self {
            text: Color::rgb(242, 222, 230),
            gloss: Color::rgb(185, 150, 165),
            pos: Color::rgb(135, 105, 118),
            fresh: Color::rgb(255, 100, 140),
            index: Color::rgb(135, 105, 118),
            cloud: Color::rgb(240, 130, 170),
            background: Color::rgb(38, 26, 32),
            highlight: Color::rgba(240, 120, 160, 90),
            border: Color::rgb(64, 48, 56),
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
