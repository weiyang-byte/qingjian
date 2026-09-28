use objc2_foundation::NSInteger;
use qingjian_core::FuzzyRules;

/// 模糊音条目的 tag 起点，后面加规则在 [`FuzzyRules::NAMES`] 里的下标。
const FUZZY_TAG_BASE: NSInteger = 100;

/// 候选窗配色条目的 tag 起点，后面加风格在 [`qingjian_platform::CandidateStyle::NAMES`] 里的下标。
const STYLE_TAG_BASE: NSInteger = 200;

/// 菜单能触发的动作。编码进 NSMenuItem 的 tag，派发时再解出来。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    /// 开关云联想（写 `[predict] enabled`）。
    ToggleCloud,

    /// 开关一条模糊音规则，值是 [`FuzzyRules::NAMES`] 的下标。
    ToggleFuzzy(usize),

    /// 切候选窗配色，值是 [`qingjian_platform::CandidateStyle::NAMES`] 的下标。
    SetStyle(usize),

    /// 打开偏好设置窗口。
    OpenPreferences,

    /// 在访达里打开日志目录。
    OpenLogs,

    /// 打开下载页（菜单里「有新版本」那一行）。
    OpenDownload,
}

impl MenuAction {
    pub fn tag(self) -> NSInteger {
        match self {
            Self::ToggleCloud => 1,
            Self::OpenPreferences => 2,
            Self::OpenLogs => 3,
            Self::OpenDownload => 4,
            Self::ToggleFuzzy(index) => FUZZY_TAG_BASE + index as NSInteger,
            Self::SetStyle(index) => STYLE_TAG_BASE + index as NSInteger,
        }
    }

    pub fn from_tag(tag: NSInteger) -> Option<Self> {
        Some(match tag {
            1 => Self::ToggleCloud,
            2 => Self::OpenPreferences,
            3 => Self::OpenLogs,
            4 => Self::OpenDownload,
            // 100 段是模糊音、200 段是配色；各自的范围检查独立做，
            // 不能用 `?` 把「不在这一段」直接返回 None，那会把另一段整个吞掉
            _ => {
                let fuzzy = tag
                    .checked_sub(FUZZY_TAG_BASE)
                    .and_then(|index| usize::try_from(index).ok())
                    .filter(|index| *index < FuzzyRules::NAMES.len());
                if let Some(index) = fuzzy {
                    return Some(Self::ToggleFuzzy(index));
                }
                let style = tag
                    .checked_sub(STYLE_TAG_BASE)
                    .and_then(|index| usize::try_from(index).ok())
                    .filter(|index| *index < qingjian_platform::CandidateStyle::NAMES.len());
                style.map(Self::SetStyle)?
            }
        })
    }

    /// 按菜单标题认动作：IMK 转发系统输入源菜单点击时 item 可能被换掉，tag 不可信，标题是对的。
    /// 配色三项的标题带「配色：」前缀。
    pub fn from_title(title: &str) -> Option<Self> {
        let label = title.strip_prefix("配色：")?;
        qingjian_platform::CandidateStyle::NAMES
            .iter()
            .enumerate()
            .find(|(_, (_, style_label))| *style_label == label)
            .map(|(index, _)| Self::SetStyle(index))
    }
}

    #[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn tags_round_trip() {
        let all = [
            MenuAction::ToggleCloud,
            MenuAction::OpenPreferences,
            MenuAction::OpenLogs,
            MenuAction::OpenDownload,
            MenuAction::ToggleFuzzy(0),
            MenuAction::ToggleFuzzy(FuzzyRules::NAMES.len() - 1),
            MenuAction::SetStyle(0),
            MenuAction::SetStyle(qingjian_platform::CandidateStyle::NAMES.len() - 1),
        ];
        for action in all {
            assert_eq!(MenuAction::from_tag(action.tag()), Some(action));
        }
        assert_eq!(MenuAction::from_tag(0), None);
        assert_eq!(
            MenuAction::from_tag(FUZZY_TAG_BASE + FuzzyRules::NAMES.len() as NSInteger),
            None
        );
        // 模糊音段查不到的 tag 要继续查配色段，不能整个吞掉
        assert_eq!(
            MenuAction::from_tag(STYLE_TAG_BASE),
            Some(MenuAction::SetStyle(0))
        );
        assert_eq!(
            MenuAction::from_tag(
                STYLE_TAG_BASE + qingjian_platform::CandidateStyle::NAMES.len() as NSInteger
            ),
            None
        );
        assert_eq!(MenuAction::from_tag(-1), None);
    }
}
