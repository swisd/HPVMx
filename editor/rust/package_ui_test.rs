pub fn render_ui(pg: &mut PixelGraphics) {
    pg.draw_text(20, 100, "Packages", 0x00FF00);
    pg.draw_button(220, 120, 30, false, "Refresh Local Table", 120);
    pg.draw_checkbox(360, 120, false, false, false, "Advanced");
    pg.draw_button(820, 120, 30, false, "Update Index", 120);
    pg.draw_tree_view_icon(60, 160, 285, 450, &root, &pixel_graphics::icons::PACKAGE_ICON_DATA);
    pg.draw_table_view(360, 160, 200, 450);
    pg.draw_table_view(660, 160, 150, 400);
    pg.draw_table_view(820, 160, 400, 500);
    pg.draw_button(660, 580, 120, 30, "Install", false);
    pg.draw_button(60, 640, 120, 30, "Uninstall", false);
    pg.draw_button(220, 640, 120, 30, "Update", false);
    pg.draw_button(380, 640, 120, 30, "Disable", false);
    pg.draw_button(660, 640, 120, 30, "____", false);
    pg.draw_rect_outline(820, 680, 300, 25, 0xCCCCC0);
    pg.draw_button(1140, 680, 100, 25, "Search", false);
}
