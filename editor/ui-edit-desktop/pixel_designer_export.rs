pub fn render_generated_ui(gfx: &mut PixelGraphics) {
    let graph_data: [u64; 10] = [8, 18, 14, 32, 28, 44, 52, 48, 70, 64];
    let list_items: [&str; 3] = ["Alpha", "Beta", "Gamma"];
    let table_headers: [&str; 2] = ["Name", "Value"];
    let table_row_a: [&str; 2] = ["CPU", "42"];
    let table_rows: [&[&str]; 1] = [&table_row_a];
    // Tree/icon exports expect caller-provided `root` and `ICON_DATA` symbols when used.

    gfx.draw_button(280, 80, 120, 30, "Click Me", false);
    gfx.draw_checkbox(280, 120, true, false, false, "Option");
    gfx.draw_table_view(420, 80, 220, 180, &table_headers, &table_rows);
    gfx.draw_table_view(660, 80, 220, 180, &table_headers, &table_rows);
    gfx.draw_tree_view(380, 280, 250, 220, &root);
    gfx.draw_tree_view(660, 280, 250, 220, &root);
    gfx.fill_rect(280, 160, 100, 50, 0x444444);
    gfx.fill_rect(380, 520, 100, 50, 0x444444);
    gfx.draw_progress_bar(940, 80, 200, 25, 30, 100, 0xFF00);
    gfx.draw_line_graph(920, 120, 300, 150, &graph_data, 100, 0xFFFF, 10);
    gfx.draw_lcd_number(1160, 80, "0000");
    gfx.draw_list_view(920, 280, 220, 180, &list_items, None);
    gfx.draw_tree_view_icon(660, 520, 250, 220, &root, &ICON_DATA);
    gfx.draw_tristate_checkbox(280, 220, "Tristate", 0xFFFFFF, false, false);
    gfx.draw_radio_button(240, 460, false);
    gfx.draw_slider(260, 240, 150, 50, 100, false);
    gfx.draw_dial(260, 280, 30, 25, 100);
    gfx.draw_spinbox(240, 360, 80, 0, "Label");
    gfx.draw_double_spinbox(240, 400, 100, 0.0, 2);
    gfx.draw_line(920, 500, 1200, 680, 0xFFFFFF);
    gfx.draw_radio_button(240, 480, false);
    gfx.draw_text(260, 460, "RB1/2", 0xFFFFFF);
    gfx.draw_rect_outline(240, 440, 100, 80, 0x444444);
}
