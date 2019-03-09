use crate::sys;
use crate::Ui;

/// Represents one of the supported mouse buttons
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Extra1 = 3,
    Extra2 = 4,
}

impl MouseButton {
    /// All possible `MouseButton` varirants
    pub const VARIANTS: [MouseButton; MouseButton::COUNT] = [
        MouseButton::Left,
        MouseButton::Right,
        MouseButton::Middle,
        MouseButton::Extra1,
        MouseButton::Extra2,
    ];
    /// Total count of `MouseButton` variants
    pub const COUNT: usize = 5;
}

/// Mouse cursor type identifier
#[repr(i32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MouseCursor {
    Arrow = sys::ImGuiMouseCursor_Arrow,
    /// Automatically used when hovering over text inputs, etc.
    TextInput = sys::ImGuiMouseCursor_TextInput,
    /// Not used automatically
    ResizeAll = sys::ImGuiMouseCursor_ResizeAll,
    /// Automatically used when hovering over a horizontal border
    ResizeNS = sys::ImGuiMouseCursor_ResizeNS,
    /// Automatically used when hovering over a vertical border or a column
    ResizeEW = sys::ImGuiMouseCursor_ResizeEW,
    /// Automatically used when hovering over the bottom-left corner of a window
    ResizeNESW = sys::ImGuiMouseCursor_ResizeNESW,
    /// Automatically used when hovering over the bottom-right corner of a window
    ResizeNWSE = sys::ImGuiMouseCursor_ResizeNWSE,
    /// Not used automatically, use for e.g. hyperlinks
    Hand = sys::ImGuiMouseCursor_Hand,
}
impl MouseCursor {
    /// All possible `MouseCursor` varirants
    pub const VARIANTS: [MouseCursor; MouseCursor::COUNT] = [
        MouseCursor::Arrow,
        MouseCursor::TextInput,
        MouseCursor::ResizeAll,
        MouseCursor::ResizeNS,
        MouseCursor::ResizeEW,
        MouseCursor::ResizeNESW,
        MouseCursor::ResizeNWSE,
        MouseCursor::Hand,
    ];
    /// Total count of `MouseCursor` variants
    pub const COUNT: usize = sys::ImGuiMouseCursor_COUNT as usize;
}

#[test]
fn test_mouse_cursor_variants() {
    for (idx, &value) in MouseCursor::VARIANTS.iter().enumerate() {
        assert_eq!(idx, value as usize);
    }
}

impl<'ui> Ui<'ui> {
    /// Returns true if the given mouse button is held down.
    ///
    /// Equivalent to indexing the Io struct with the button, e.g. `ui.io()[button]`.
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDown(button as i32) }
    }
    /// Returns true if any mouse button is held down
    pub fn is_any_mouse_down(&self) -> bool {
        unsafe { sys::igIsAnyMouseDown() }
    }
    /// Returns true if the given mouse button was clicked (went from !down to down)
    pub fn is_mouse_clicked(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseClicked(button as i32, false) }
    }
    /// Returns true if the given mouse button was double-clicked
    pub fn is_mouse_double_clicked(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDoubleClicked(button as i32) }
    }
    /// Returns true if the given mouse button was released (went from down to !down)
    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseReleased(button as i32) }
    }
    /// Returns true if the mouse is currently dragging with the given mouse button held down
    pub fn is_mouse_dragging(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDragging(button as i32, -1.0) }
    }
    /// Returns true if the mouse is currently dragging with the given mouse button held down.
    ///
    /// If the given threshold is invalid or negative, the global distance threshold is used
    /// (`io.mouse_drag_threshold`).
    pub fn is_mouse_dragging_with_threshold(&self, button: MouseButton, threshold: f32) -> bool {
        unsafe { sys::igIsMouseDragging(button as i32, threshold) }
    }
    /// Returns true if the mouse is hovering over the given bounding rect.
    ///
    /// Clipped by current clipping settings, but disregards other factors like focus, window
    /// ordering, modal popup blocking.
    pub fn is_mouse_hovering_rect(r_min: [f32; 2], r_max: [f32; 2]) -> bool {
        unsafe { sys::igIsMouseHoveringRect(r_min.into(), r_max.into(), true) }
    }
    /// Returns the mouse position backed up at the time of opening a popup
    pub fn get_mouse_pos_on_opening_current_popup(&self) -> [f32; 2] {
        unsafe { sys::igGetMousePosOnOpeningCurrentPopup_nonUDT2().into() }
    }
    /// Returns the delta from the initial clicking position.
    ///
    /// This is locked and returns [0.0, 0.0] until the mouse has moved past the global distance
    /// threshold (`io.mouse_drag_threshold`).
    pub fn get_mouse_drag_delta(&self, button: MouseButton) -> [f32; 2] {
        unsafe { sys::igGetMouseDragDelta_nonUDT2(button as i32, -1.0).into() }
    }
    /// Returns the delta from the initial clicking position.
    ///
    /// This is locked and returns [0.0, 0.0] until the mouse has moved past the given threshold.
    /// If the given threshold is invalid or negative, the global distance threshold is used
    /// (`io.mouse_drag_threshold`).
    pub fn get_mouse_drag_delta_with_threshold(
        &self,
        button: MouseButton,
        threshold: f32,
    ) -> [f32; 2] {
        unsafe { sys::igGetMouseDragDelta_nonUDT2(button as i32, threshold).into() }
    }
    /// Resets the current delta from initial clicking position.
    pub fn reset_mouse_drag_delta(&self, button: MouseButton) {
        // This mutates the Io struct, but targets an internal field so there can't be any
        // references to it
        unsafe { sys::igResetMouseDragDelta(button as i32) }
    }
    /// Get the currently desired mouse cursor type.
    ///
    /// Returns `None` if no cursor should be displayed
    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        match unsafe { sys::igGetMouseCursor() } {
            sys::ImGuiMouseCursor_Arrow => Some(MouseCursor::Arrow),
            sys::ImGuiMouseCursor_TextInput => Some(MouseCursor::TextInput),
            sys::ImGuiMouseCursor_ResizeAll => Some(MouseCursor::ResizeAll),
            sys::ImGuiMouseCursor_ResizeNS => Some(MouseCursor::ResizeNS),
            sys::ImGuiMouseCursor_ResizeEW => Some(MouseCursor::ResizeEW),
            sys::ImGuiMouseCursor_ResizeNESW => Some(MouseCursor::ResizeNESW),
            sys::ImGuiMouseCursor_ResizeNWSE => Some(MouseCursor::ResizeNWSE),
            sys::ImGuiMouseCursor_Hand => Some(MouseCursor::Hand),
            _ => None,
        }
    }
    /// Set the desired mouse cursor type.
    ///
    /// Passing `None` hides the mouse cursor.
    pub fn set_mouse_cursor(&self, cursor_type: Option<MouseCursor>) {
        unsafe {
            sys::igSetMouseCursor(
                cursor_type
                    .map(|x| x as i32)
                    .unwrap_or(sys::ImGuiMouseCursor_None),
            );
        }
    }
}

#[test]
fn test_mouse_down_clicked_released() {
    for &button in MouseButton::VARIANTS.iter() {
        let (_guard, mut ctx) = crate::test::test_ctx_initialized();
        {
            ctx.io_mut().mouse_down = [false; 5];
            let ui = ctx.frame();
            assert!(!ui.is_mouse_down(button));
            assert!(!ui.is_any_mouse_down());
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_released(button));
        }
        {
            ctx.io_mut()[button] = true;
            let ui = ctx.frame();
            assert!(ui.is_mouse_down(button));
            assert!(ui.is_any_mouse_down());
            assert!(ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_released(button));
        }
        {
            let ui = ctx.frame();
            assert!(ui.is_mouse_down(button));
            assert!(ui.is_any_mouse_down());
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_released(button));
        }
        {
            ctx.io_mut()[button] = false;
            let ui = ctx.frame();
            assert!(!ui.is_mouse_down(button));
            assert!(!ui.is_any_mouse_down());
            assert!(!ui.is_mouse_clicked(button));
            assert!(ui.is_mouse_released(button));
        }
        {
            let ui = ctx.frame();
            assert!(!ui.is_mouse_down(button));
            assert!(!ui.is_any_mouse_down());
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_released(button));
        }
    }
}

#[test]
fn test_mouse_double_click() {
    let (_guard, mut ctx) = crate::test::test_ctx_initialized();
    // Workaround for dear imgui bug/feature:
    // If a button is clicked before io.mouse_double_click_time seconds has passed after the
    // context is initialized, the single click is interpreted as a double-click.  This happens
    // because internally g.IO.MouseClickedTime is set to 0.0, so the context creation is
    // considered a "click".
    {
        // Pass one second of time
        ctx.io_mut().delta_time = 1.0;
        let _ = ctx.frame();
    }
    // Fast clicks
    ctx.io_mut().delta_time = 1.0 / 60.0;
    for &button in MouseButton::VARIANTS.iter() {
        {
            ctx.io_mut().mouse_down = [false; 5];
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = true;
            let ui = ctx.frame();
            assert!(ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = false;
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = true;
            let ui = ctx.frame();
            assert!(ui.is_mouse_clicked(button));
            assert!(ui.is_mouse_double_clicked(button));
        }
        {
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
    }
    // Slow clicks
    ctx.io_mut().delta_time = 1.0;
    for &button in MouseButton::VARIANTS.iter() {
        {
            ctx.io_mut().mouse_down = [false; 5];
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = true;
            let ui = ctx.frame();
            assert!(ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = false;
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            ctx.io_mut()[button] = true;
            let ui = ctx.frame();
            assert!(ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
        {
            let ui = ctx.frame();
            assert!(!ui.is_mouse_clicked(button));
            assert!(!ui.is_mouse_double_clicked(button));
        }
    }
}

#[test]
fn test_set_get_mouse_cursor() {
    let (_guard, mut ctx) = crate::test::test_ctx_initialized();
    let ui = ctx.frame();
    ui.set_mouse_cursor(None);
    assert_eq!(None, ui.mouse_cursor());
    ui.set_mouse_cursor(Some(MouseCursor::Hand));
    assert_eq!(Some(MouseCursor::Hand), ui.mouse_cursor());
}
