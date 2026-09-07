//! Helpers for handling GPUI's portable touch-drag gestures in the example.
//!
//! GPUI core recognizes gestures from raw `TouchEvent`s.  A pan that exceeds
//! the touch slop is offered to elements as a `TouchDragEvent`; an element
//! that calls `prevent_default()` on the `Started` phase claims the drag and
//! receives the subsequent `Moved`/`Ended` phases.  Otherwise the gesture
//! falls through to scrolling (as `ScrollWheel` events).
//!
//! This module provides a small helper that wires that up for an element,
//! filtering drags to a given bounds.

use gpui::{App, Bounds, DispatchPhase, Pixels, TouchDragEvent, TouchPhase, Window};

/// Registers a `TouchDragEvent` listener on `window` that only handles drags
/// which start inside `bounds`.
///
/// It claims the drag (via `prevent_default`) so the touch becomes a direct
/// drag rather than a scroll, and forwards the `Started`/`Moved`/`Ended`
/// phases to `on_drag`.
///
/// Must be called during the paint phase (e.g. inside a `canvas` callback),
/// where `bounds` is the element's laid-out bounds.
pub fn handle_touch_drag<F>(window: &mut Window, bounds: Bounds<Pixels>, mut on_drag: F)
where
    F: FnMut(&TouchDragEvent, &mut Window, &mut App) + 'static,
{
    window.on_mouse_event(move |event: &TouchDragEvent, phase, window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if !bounds.contains(&event.start_position) {
            return;
        }
        if event.phase == TouchPhase::Started {
            window.prevent_default();
        }
        on_drag(event, window, cx);
    });
}
