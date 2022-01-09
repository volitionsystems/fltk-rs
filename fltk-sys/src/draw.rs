/* automatically generated by rust-bindgen 0.57.0 */

extern "C" {
    pub fn Fl_set_color_int(c: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_set_color_rgb(
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_color() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_push_clip(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_push_no_clip();
}
extern "C" {
    pub fn Fl_pop_clip();
}
extern "C" {
    pub fn Fl_not_clipped(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_clip_box(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        X: *mut ::std::os::raw::c_int,
        Y: *mut ::std::os::raw::c_int,
        W: *mut ::std::os::raw::c_int,
        H: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_restore_clip();
}
extern "C" {
    pub fn Fl_set_clip_region(r: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_clip_region() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_point(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_line_style(
        style: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        dashes: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_rect(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_focus_rect(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_rect_with_color(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        c: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_rectf_with_color(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        c: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf_with_rgb(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        r: ::std::os::raw::c_uchar,
        g: ::std::os::raw::c_uchar,
        b: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Fl_line(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_line2(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_loop(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_loop2(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        x3: ::std::os::raw::c_int,
        y3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon2(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        x3: ::std::os::raw::c_int,
        y3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_xyline(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int, x1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_xyline2(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_xyline3(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        x1: ::std::os::raw::c_int,
        y2: ::std::os::raw::c_int,
        x3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_yxline(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int, y1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_yxline2(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_yxline3(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        y1: ::std::os::raw::c_int,
        x2: ::std::os::raw::c_int,
        y3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_arc(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        a1: f64,
        a2: f64,
    );
}
extern "C" {
    pub fn Fl_pie(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        a1: f64,
        a2: f64,
    );
}
extern "C" {
    pub fn Fl_push_matrix();
}
extern "C" {
    pub fn Fl_pop_matrix();
}
extern "C" {
    pub fn Fl_scale(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_scale2(x: f64);
}
extern "C" {
    pub fn Fl_translate(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_rotate(d: f64);
}
extern "C" {
    pub fn Fl_mult_matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64);
}
extern "C" {
    pub fn Fl_begin_points();
}
extern "C" {
    pub fn Fl_begin_line();
}
extern "C" {
    pub fn Fl_begin_loop();
}
extern "C" {
    pub fn Fl_begin_polygon();
}
extern "C" {
    pub fn Fl_vertex(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_curve(X0: f64, Y0: f64, X1: f64, Y1: f64, X2: f64, Y2: f64, X3: f64, Y3: f64);
}
extern "C" {
    pub fn Fl_arc2(x: f64, y: f64, r: f64, start: f64, end: f64);
}
extern "C" {
    pub fn Fl_circle(x: f64, y: f64, r: f64);
}
extern "C" {
    pub fn Fl_end_points();
}
extern "C" {
    pub fn Fl_end_line();
}
extern "C" {
    pub fn Fl_end_loop();
}
extern "C" {
    pub fn Fl_end_polygon();
}
extern "C" {
    pub fn Fl_begin_complex_polygon();
}
extern "C" {
    pub fn Fl_gap();
}
extern "C" {
    pub fn Fl_end_complex_polygon();
}
extern "C" {
    pub fn Fl_transform_x(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_y(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dx(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dy(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transformed_vertex(xf: f64, yf: f64);
}
extern "C" {
    pub fn Fl_end_offscreen();
}
extern "C" {
    pub fn Fl_set_draw_font(face: ::std::os::raw::c_int, fsize: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_font() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_size() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_height() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_set_height(
        font: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_descent() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_width(txt: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn Fl_width2(txt: *const ::std::os::raw::c_char, n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn Fl_width3(c: ::std::os::raw::c_uint) -> f64;
}
extern "C" {
    pub fn Fl_text_extents(
        arg1: *const ::std::os::raw::c_char,
        dx: *mut ::std::os::raw::c_int,
        dy: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_text_extents2(
        t: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
        dx: *mut ::std::os::raw::c_int,
        dy: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_latin1_to_local(
        t: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_local_to_latin1(
        t: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_mac_roman_to_local(
        t: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_local_to_mac_roman(
        t: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_draw(
        str_: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw2(
        angle: ::std::os::raw::c_int,
        str_: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw3(
        str_: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw4(
        angle: ::std::os::raw::c_int,
        str_: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_rtl_draw(
        str_: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_measure(
        str_: *const ::std::os::raw::c_char,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
        draw_symbols: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw5(
        str_: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
        img: *mut *mut ::std::os::raw::c_void,
        draw_symbols: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_frame(
        s: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_frame2(
        s: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box(
        box_type: ::std::os::raw::c_int,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        arg1: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_draw_image(
        buf: *const ::std::os::raw::c_uchar,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        D: ::std::os::raw::c_int,
        L: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_image_mono(
        buf: *const ::std::os::raw::c_uchar,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        D: ::std::os::raw::c_int,
        L: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_can_do_alpha_blending() -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_read_image(
        p: *mut ::std::os::raw::c_uchar,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        alpha: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_capture_window_part(
        win: *mut ::std::os::raw::c_void,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_draw_pixmap(
        data: *const *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        bg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_draw_pixmap2(
        data: *const *mut ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        bg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap(
        data: *const *mut ::std::os::raw::c_char,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap2(
        cdata: *const *const ::std::os::raw::c_char,
        w: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_shortcut_label(shortcut: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_shortcut_label2(
        shortcut: ::std::os::raw::c_uint,
        eom: *mut *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_old_shortcut(s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_overlay_rect(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_overlay_clear();
}
extern "C" {
    pub fn Fl_set_cursor(cursor: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_set_cursor2(
        cursor: ::std::os::raw::c_int,
        fg: ::std::os::raw::c_int,
        bg: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_expand_text(
        from: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        maxbuf: ::std::os::raw::c_int,
        maxw: f64,
        n: *mut ::std::os::raw::c_int,
        width: *mut f64,
        wrap: ::std::os::raw::c_int,
        draw_symbols: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_set_status(
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_set_spot(
        font: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        win: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_reset_spot();
}
extern "C" {
    pub fn Fl_show_colormap(old_col: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_copy_offscreen(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        pixmap: *mut ::std::os::raw::c_void,
        srcx: ::std::os::raw::c_int,
        srcy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_create_offscreen(
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_begin_offscreen(b: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_delete_offscreen(bitmap: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_rescale_offscreen(ctx: *mut *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_draw_text2(
        str_: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_check(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        col: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_add_symbol(
        name: *const ::std::os::raw::c_char,
        drawit: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_uint)>,
        scalable: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
