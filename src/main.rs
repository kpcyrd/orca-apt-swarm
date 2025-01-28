use std::io;

use ratzilla::ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{self, canvas},
    Frame, Terminal,
};

use ratzilla::{DomBackend, WebRenderer};

fn render_map(frame: &mut Frame<'_>, area: Rect) {
    let canvas = canvas::Canvas::default()
        .marker(Marker::HalfBlock)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&canvas::Map {
                resolution: canvas::MapResolution::High,
                color: Color::Rgb(0, 118, 0),
            });

            let red = Color::Rgb(174, 31, 31);
            // germany
            ctx.print(10.0, 48.0, "X".fg(red).bold());
            // ukraine
            ctx.print(22.0, 47.0, "X".fg(red).bold());
            // south africa
            ctx.print(28.0, -26.0, "X".fg(red).bold());
            // saudi arabia
            ctx.print(45.0, 24.0, "X".fg(red).bold());
            // hong kong
            ctx.print(115.0, 23.0, "X".fg(red).bold());
            // miami
            ctx.print(-81.0, 26.0, "X".fg(red).bold());
            // mexico
            ctx.print(-100.0, 20.0, "X".fg(red).bold());
            // hawaii
            ctx.print(-156.0, 19.0, "X".fg(red).bold());
        });
    frame.render_widget(canvas, area);
}

/*
fn render_footer(frame: &mut Frame<'_>, area: Rect) {
    use ratzilla::ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
    use ratzilla::ratatui::text::{Line, Span, Text};
    use ratzilla::widgets::Hyperlink;
    let link = Hyperlink::new("https://github.com/kpcyrd/orca-apt-swarm");

    let container = widgets::Block::default().style(Style::default().fg(Color::DarkGray));
    frame.render_widget(&container, area);
    let area = container.inner(area);

    frame.render_widget(link, area);
}
*/

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.draw_web(move |f| {
        let area = f.area();
        let container = widgets::Block::bordered()
            .title("apt-swarm p2p node locations")
            .border_style(Style::new().gray());
        f.render_widget(&container, area);
        let area = container.inner(area);

        /*
        let vertical =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).flex(Flex::Center);
        let [map, footer] = vertical.areas(area);
        render_map(f, map);
        render_footer(f, footer);
        */
        render_map(f, area);
    });
    Ok(())
}
