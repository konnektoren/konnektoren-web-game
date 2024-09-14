use web_sys::window;

pub(crate) fn adjust_info_position(
    x: f64,
    y: f64,
    dialog_width: f64,
    dialog_height: f64,
) -> (f64, f64) {
    let window = window().unwrap();
    let window_width = window.inner_width().unwrap().as_f64().unwrap();
    let window_height = window.inner_height().unwrap().as_f64().unwrap();

    let mut adjusted_x = x;
    let mut adjusted_y = y;

    // If the dialog goes beyond the right edge of the window, adjust x
    if adjusted_x + dialog_width > window_width {
        adjusted_x = window_width - dialog_width;
    }
    // Ensure x is not negative
    if adjusted_x < 0.0 {
        adjusted_x = 0.0;
    }

    // If the dialog goes beyond the bottom edge of the window, adjust y
    if adjusted_y + dialog_height > window_height {
        adjusted_y = window_height - dialog_height;
    }
    // Ensure y is not negative
    if adjusted_y < 0.0 {
        adjusted_y = 0.0;
    }

    (adjusted_x, adjusted_y)
}
