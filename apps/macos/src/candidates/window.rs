//! 候选窗口：非激活的浮动 NSPanel，跟随光标，内容由 [`CandidateView`] 绘制。

use objc2::AnyThread;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSAppearanceNameAqua, NSAppearanceNameDarkAqua,
    NSAutoresizingMaskOptions, NSBackingStoreType, NSBezierPath, NSColor, NSEvent, NSPanel,
    NSScreen, NSVisualEffectBlendingMode, NSVisualEffectMaterial, NSVisualEffectState,
    NSImage, NSVisualEffectView, NSWindowCollectionBehavior, NSWindowLevel, NSWindowStyleMask,
};
use objc2_foundation::{NSEdgeInsets, NSPoint, NSRect, NSSize};
use qingjian_platform::{CandidateRenderer, CandidateStyle, LayoutMode, ThemeMode};

use super::frame::Frame;
use super::theme::Theme;
use super::view::CandidateView;

/// `kCGPopUpMenuWindowLevel`：浮在普通窗口和浮动面板之上，与系统输入法候选框同级。
const POPUP_MENU_LEVEL: NSWindowLevel = 101;

/// 候选窗口与光标行之间的间隙。
const CARET_GAP: f64 = 4.0;

/// 应用给不出光标位置时，拿鼠标位置当光标，按这个高度算一行。
const FALLBACK_LINE_HEIGHT: f64 = 16.0;

pub struct CandidateWindow {
    /// 面板本体。不在当前 Space 时会整个换新（见 [`Self::order_front_on_active_space`]）。
    panel: Retained<NSPanel>,

    /// 内容视图；换面板时搬过去。
    view: Retained<CandidateView>,

    /// 当前外观（跟随系统时为 `None`）；换面板时要重设。
    appearance: Option<Retained<NSAppearance>>,

    /// 毛玻璃底层视图：`frosted` 风格时垫在内容视图下面；换面板时要重装。
    vibrancy: Option<Retained<NSVisualEffectView>>,

    /// 用来取屏幕尺寸。
    mtm: MainThreadMarker,
}

impl CandidateWindow {
    pub fn new(mtm: MainThreadMarker) -> Self {
        let view = CandidateView::new(mtm, Theme::system_default());
        let panel = build_panel(mtm, &view);
        Self {
            panel,
            view,
            appearance: None,
            vibrancy: None,
            mtm,
        }
    }

    /// 显示一帧。`anchor` 是光标行的屏幕矩形，窗口贴在它下方，放不下就放上方。
    pub fn show(&mut self, frame: Frame, anchor: NSRect) {
        if frame.is_empty() {
            self.hide();
            return;
        }
        let size = self.view.set_frame(&frame);
        let origin = self.place(size, anchor);
        self.panel.setFrame_display(NSRect::new(origin, size), true);
        self.order_front_on_active_space();
        // 上屏之后 layer 一定已建，补设圆角裁切（安装时窗口未上屏会拿不到 layer）
        self.apply_effect_rounding();
        if !self.panel.isVisible() {
            tracing::warn!(?anchor, ?origin, "候选窗口 orderFront 之后仍不可见");
        } else {
            tracing::debug!(
                ?anchor,
                ?origin,
                on_active_space = self.panel.isOnActiveSpace(),
                "候选窗口已显示"
            );
        }
    }

    pub fn hide(&self) {
        self.panel.orderOut(None);
    }

    /// 排到最前，并确认真在当前 Space 上；不在就换一块新面板。
    ///
    /// collection behavior 是「所有 Space + 全屏辅助」，可 macOS 26 上 WindowServer 只把面板绑到它创建时已有的 Space：
    /// 之后新开的全屏 Space（Zed 全屏就是一个）里没有它，候选框留在桌面那个 Space。用 CGS 查过，面板的 Space 列表只有桌面，
    /// 而同样设置新建的面板会绑到全部 Space；重设 collection behavior、收起再排前都不会让它重算，只有新建有用。
    /// `isOnActiveSpace` 能反映这个状态，所以每次显示后查一下，不在就把内容视图搬到新面板上。
    fn order_front_on_active_space(&mut self) {
        self.panel.orderFrontRegardless();
        if self.panel.isOnActiveSpace() {
            return;
        }
        let frame = self.panel.frame();
        self.panel.orderOut(None);
        let panel = build_panel(self.mtm, &self.view);
        panel.setAppearance(self.appearance.as_deref());
        panel.setFrame_display(frame, true);
        panel.orderFrontRegardless();
        self.panel = panel;
        // 新面板的内容视图是裸的 CandidateView，毛玻璃要重装
        self.reinstall_vibrancy();
        tracing::warn!(
            recovered = self.panel.isOnActiveSpace(),
            "候选窗口不在当前 Space，已换新面板"
        );
    }

    /// 外观：跟随系统时不指定，否则强制浅色 / 深色。
    pub fn set_theme(&mut self, mode: ThemeMode) {
        // SAFETY: 只读 AppKit 导出的常量名
        let name = unsafe {
            match mode {
                ThemeMode::System => None,
                ThemeMode::Light => Some(NSAppearanceNameAqua),
                ThemeMode::Dark => Some(NSAppearanceNameDarkAqua),
            }
        };
        let appearance = name.and_then(NSAppearance::appearanceNamed);
        self.panel.setAppearance(appearance.as_deref());
        self.appearance = appearance;
    }

    /// 竖排 / 横排。下一帧生效。
    pub fn set_layout(&self, layout: LayoutMode) {
        self.view.set_layout(layout);
    }

    /// 青简渲染器 / 系统绘制。下一帧生效。
    pub fn set_renderer(&self, renderer: CandidateRenderer) {
        self.view.set_renderer(renderer);
    }

    /// 候选窗字体（字族名，空为系统字体），只对青简渲染器生效。
    pub fn set_font(&self, font: &str) {
        self.view.set_font(font);
    }

    /// 配色风格：视图换调色板，`frosted` 再垫一层系统毛玻璃材质。
    pub fn set_style(&mut self, style: CandidateStyle) {
        let vibrancy = style == CandidateStyle::Frosted;
        self.view.set_style(style);
        if vibrancy == self.vibrancy.is_some() {
            return;
        }
        if vibrancy {
            self.install_vibrancy();
        } else {
            self.remove_vibrancy();
        }
    }

    /// 装 / 卸毛玻璃层，按当前状态对齐。
    fn reinstall_vibrancy(&mut self) {
        if self.vibrancy.is_some() {
            self.install_vibrancy();
        }
    }

    /// 把毛玻璃视图垫到内容视图下面：效果视图当 contentView 自动随窗缩放，
    /// CandidateView 变成它的子视图，装好后再按效果视图的实际边界摆位。
    fn install_vibrancy(&mut self) {
        let effect = NSVisualEffectView::initWithFrame(
            self.mtm.alloc::<NSVisualEffectView>(),
            NSRect::ZERO,
        );
        // HudWindow：深色玻璃。材质跟随外观，浅色模式下会渲染成浅磨砂——强制深色外观，
        // 让 HUD 在任何系统外观下都是深色玻璃，与 frosted 的深色位图色调一致
        effect.setMaterial(NSVisualEffectMaterial::HUDWindow);
        if let Some(dark) = NSAppearance::appearanceNamed(unsafe { NSAppearanceNameDarkAqua }) {
            effect.setAppearance(Some(&dark));
        }
        effect.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        effect.setState(NSVisualEffectState::Active);
        // BehindWindow 的模糊由窗口服务按矩形区域做，layer 的圆角裁切管不到它——
        // 必须用官方的 maskImage 把材质（含模糊区）裁成圆角，否则圆角玻璃外一圈方形残影
        effect.setMaskImage(Some(&Self::rounded_mask_image()));
        effect.setWantsLayer(true);
        self.panel.setContentView(Some(&effect));
        effect.addSubview(&self.view);
        // contentView 装进窗口后才拿得到真实边界；autoresizing 负责之后的跟随
        self.view.setFrame(effect.bounds());
        self.view
            .setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable |
                NSAutoresizingMaskOptions::ViewHeightSizable);
        self.vibrancy = Some(effect);
        self.apply_effect_rounding();
        let frame = self
            .vibrancy
            .as_ref()
            .map(|v| v.frame())
            .unwrap_or(NSRect::ZERO);
        let content_subviews = self
            .panel
            .contentView()
            .map(|v| v.subviews().len());
        tracing::info!(?frame, ?content_subviews, "候选窗已垫毛玻璃材质");
    }

    /// 拿掉毛玻璃：CandidateView 重新当 contentView，自动随窗缩放。
    fn remove_vibrancy(&mut self) {
        self.vibrancy = None;
        self.panel.setContentView(Some(&self.view));
    }

    /// 材质圆角蒙版：黑 = 显示、透明 = 裁掉。60pt 见方、圆角 10pt，四边 20pt cap inset，
    /// 拉伸时四角形状不变。lockFocus 是画静态蒙版最省事的写法，弃用警告忽略。
    #[allow(deprecated)]
    fn rounded_mask_image() -> Retained<NSImage> {
        let size = NSSize::new(60.0, 60.0);
        let image = NSImage::initWithSize(NSImage::alloc(), size);
        image.lockFocus();
        NSColor::blackColor().set();
        NSBezierPath::bezierPathWithRoundedRect_xRadius_yRadius(
            NSRect::new(NSPoint::ZERO, size),
            10.0,
            10.0,
        )
        .fill();
        image.unlockFocus();
        image.setCapInsets(NSEdgeInsets {
            top: 20.0,
            left: 20.0,
            bottom: 20.0,
            right: 20.0,
        });
        image
    }

    /// 给材质层设圆角裁切。安装时 layer 可能还没建（窗口未上屏），所以每次显示都补一遍，幂等。
    fn apply_effect_rounding(&self) {
        let Some(effect) = &self.vibrancy else {
            return;
        };
        let radius = self.view.theme().corner_radius;
        if let Some(layer) = effect.layer() {
            layer.setCornerRadius(radius);
            layer.setMasksToBounds(true);
        }
    }

    pub fn max_rows(&self) -> usize {
        self.view.theme().max_rows
    }

    /// 窗口左下角坐标：贴在光标行下方；下方放不下放上方；不出光标所在的那块屏幕。
    /// 光标矩形是零或落在所有屏幕之外（应用不支持、或给的是胡话）时以鼠标位置为准，至少落在用户看着的屏幕上。
    fn place(&self, size: NSSize, anchor: NSRect) -> NSPoint {
        let (anchor, screen) = match screen_containing(self.mtm, anchor.origin) {
            Some(screen) if !(anchor.size.height == 0.0 && anchor.origin == NSPoint::ZERO) => {
                (anchor, screen)
            }
            _ => {
                let mouse = NSEvent::mouseLocation();
                let anchor = NSRect::new(mouse, NSSize::new(0.0, FALLBACK_LINE_HEIGHT));
                (
                    anchor,
                    screen_containing(self.mtm, mouse)
                        .unwrap_or_else(|| main_screen_or_anywhere(self.mtm)),
                )
            }
        };
        let min_x = screen.origin.x;
        let max_x = (screen.origin.x + screen.size.width - size.width).max(min_x);
        let x = anchor.origin.x.clamp(min_x, max_x);
        let below = anchor.origin.y - CARET_GAP - size.height;
        let above = anchor.origin.y + anchor.size.height + CARET_GAP;
        let top = screen.origin.y + screen.size.height;
        let y = if below >= screen.origin.y {
            below
        } else if above + size.height <= top {
            above
        } else {
            // 上下都放不下（屏幕很矮或窗口很高）：贴屏幕底边，宁可盖住光标也别出屏
            screen.origin.y
        };
        // 无论怎么算，最后都要落在这块屏幕里：出屏等于不显示
        let max_y = (top - size.height).max(screen.origin.y);
        NSPoint::new(x, y.clamp(screen.origin.y, max_y))
    }
}

/// 建一块面板并把内容视图装进去：无边框、不抢焦点、透明背景带阴影、不吃鼠标。
fn build_panel(mtm: MainThreadMarker, view: &CandidateView) -> Retained<NSPanel> {
    let panel = NSPanel::initWithContentRect_styleMask_backing_defer(
        mtm.alloc::<NSPanel>(),
        NSRect::new(NSPoint::ZERO, NSSize::new(200.0, 100.0)),
        NSWindowStyleMask::Borderless | NSWindowStyleMask::NonactivatingPanel,
        NSBackingStoreType::Buffered,
        false,
    );
    panel.setOpaque(false);
    panel.setBackgroundColor(Some(&NSColor::clearColor()));
    panel.setHasShadow(true);
    panel.setBecomesKeyOnlyIfNeeded(true);
    panel.setIgnoresMouseEvents(true);
    // NSPanel 缺省在应用失活时自动隐藏；输入法进程从来不是前台应用，不能靠这个
    panel.setHidesOnDeactivate(false);
    panel.setCollectionBehavior(collection_behavior());
    // 不要 setFloatingPanel(true)：它会把层级改回 NSFloatingWindowLevel（3），全屏应用的 Space 里就看不见了；
    // 层级最后设，别被前面任何一项覆盖
    panel.setLevel(POPUP_MENU_LEVEL);
    panel.setContentView(Some(view));
    panel
}

/// 面板的 Space 归属：出现在所有 Space 上、能与全屏应用同处一个 Space、Mission Control 里不动。
fn collection_behavior() -> NSWindowCollectionBehavior {
    NSWindowCollectionBehavior::CanJoinAllSpaces
        | NSWindowCollectionBehavior::FullScreenAuxiliary
        | NSWindowCollectionBehavior::Stationary
}

/// 包含 `point` 的那块屏幕的可见区域；哪块都不包含返回 `None`。
fn screen_containing(mtm: MainThreadMarker, point: NSPoint) -> Option<NSRect> {
    let screens = NSScreen::screens(mtm);
    let hit = screens.iter().find(|screen| {
        let frame = screen.frame();
        point.x >= frame.origin.x
            && point.x < frame.origin.x + frame.size.width
            && point.y >= frame.origin.y
            && point.y < frame.origin.y + frame.size.height
    });
    hit.map(|screen| screen.visibleFrame())
}

/// 主屏的可见区域；连主屏都没有时不限制。
fn main_screen_or_anywhere(mtm: MainThreadMarker) -> NSRect {
    NSScreen::mainScreen(mtm)
        .map(|s| s.visibleFrame())
        .unwrap_or_else(|| {
            NSRect::new(
                NSPoint::new(f64::MIN / 2.0, f64::MIN / 2.0),
                NSSize::new(f64::MAX, f64::MAX),
            )
        })
}
