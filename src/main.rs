mod hd;

use ratzilla::{
    WebRenderer,
    ratatui::{
        Frame, Terminal,
        layout::Rect,
        style::{Color, Style, Stylize},
        symbols::Marker,
        widgets::{self, canvas},
    },
};
use std::cmp;
use std::io;

const RED: Color = Color::Rgb(174, 31, 31);

fn render_map(frame: &mut Frame<'_>, area: Rect) {
    let canvas = canvas::Canvas::default()
        .marker(Marker::HalfBlock)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-126.0, 126.0])
        .paint(|ctx| {
            ctx.draw(&canvas::Map {
                resolution: canvas::MapResolution::High,
                color: Color::Rgb(0, 118, 0),
            });

            let red = Color::Rgb(174, 31, 31);
            // helsinki
            ctx.print(24.9, 60.1, "X".fg(red).bold());
            // ukraine
            ctx.print(30.5, 50.4, "X".fg(red).bold());
            // south africa
            ctx.print(28.0, -26.2, "X".fg(red).bold());
            // saudi arabia
            ctx.print(46.7, 24.6, "X".fg(red).bold());
            // hong kong
            ctx.print(114.1, 22.2, "X".fg(red).bold());
            // miami
            ctx.print(-80.2, 25.8, "X".fg(red).bold());
            // mexico
            ctx.print(-100.3, 20.6, "X".fg(red).bold());
            // hawaii
            ctx.print(-157.8, 21.3, "X".fg(red).bold());
            // kazakhstan
            ctx.print(76.9, 43.2, "X".fg(red).bold());
            // russia
            ctx.print(37.6, 55.0, "X".fg(red).bold());
            // australia
            ctx.print(151.2, -33.9, "X".fg(red).bold());
            // brazil
            ctx.print(-46.3, -23.5, "X".fg(red).bold());
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

fn square(area: Rect) -> Rect {
    let normalized_width = area.width / 2;
    let capped_height = cmp::min(normalized_width, area.height);
    let padding_offset = (area.height - capped_height) / 2;
    Rect {
        x: area.x,
        y: area.y + padding_offset,
        width: area.width,
        height: capped_height,
    }
}

fn main() -> io::Result<()> {
    let backend = hd::HdBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.draw_web(move |f| {
        let area = f.area();
        let area = square(area);

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
