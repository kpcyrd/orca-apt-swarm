use ratzilla::error::Error;
use ratzilla::ratatui::{
    backend::WindowSize,
    buffer::Cell,
    layout::{Position, Size},
    prelude::Backend,
    style::{Color, Modifier},
};
use ratzilla::utils;
use std::{cell::RefCell, io::Result as IoResult, rc::Rc};
use web_sys::{
    Document, Element, Window,
    wasm_bindgen::{JsCast, prelude::Closure},
    window,
};

const MULTIPLIER: u16 = 2;

// a lot of code in here is copied from the ratzilla repo

pub(crate) fn create_span(document: &Document, cell: &Cell) -> Result<Element, Error> {
    let span = document.create_element("span")?;
    span.set_inner_html(cell.symbol());

    let style = get_cell_style_as_css(cell);
    span.set_attribute("style", &style)?;
    Ok(span)
}

pub(crate) fn get_cell_style_as_css(cell: &Cell) -> String {
    let fg = ansi_to_rgb(cell.fg);
    let bg = ansi_to_rgb(cell.bg);

    let fg_style = match fg {
        Some(color) => format!("color: rgb({}, {}, {});", color.0, color.1, color.2),
        None => "color: rgb(255, 255, 255);".to_string(),
    };

    let bg_style = match bg {
        Some(color) => format!(
            "background-color: rgb({}, {}, {});",
            color.0, color.1, color.2
        ),
        None => "background-color: transparent;".to_string(),
    };

    let mut modifier_style = String::new();
    if cell.modifier.contains(Modifier::BOLD) {
        modifier_style.push_str("font-weight: bold; ");
    }
    if cell.modifier.contains(Modifier::DIM) {
        modifier_style.push_str("opacity: 0.5; ");
    }
    if cell.modifier.contains(Modifier::ITALIC) {
        modifier_style.push_str("font-style: italic; ");
    }
    if cell.modifier.contains(Modifier::UNDERLINED) {
        modifier_style.push_str("text-decoration: underline; ");
    }
    if cell.modifier.contains(Modifier::HIDDEN) {
        modifier_style.push_str("visibility: hidden; ");
    }
    if cell.modifier.contains(Modifier::CROSSED_OUT) {
        modifier_style.push_str("text-decoration: line-through; ");
    }

    format!("{fg_style} {bg_style} {modifier_style}")
}

fn ansi_to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Black => Some((0, 0, 0)),
        Color::Red => Some((128, 0, 0)),
        Color::Green => Some((0, 128, 0)),
        Color::Yellow => Some((128, 128, 0)),
        Color::Blue => Some((0, 0, 128)),
        Color::Magenta => Some((128, 0, 128)),
        Color::Cyan => Some((0, 128, 128)),
        Color::Gray => Some((192, 192, 192)),
        Color::DarkGray => Some((128, 128, 128)),
        Color::LightRed => Some((255, 0, 0)),
        Color::LightGreen => Some((0, 255, 0)),
        Color::LightYellow => Some((255, 255, 0)),
        Color::LightBlue => Some((0, 0, 255)),
        Color::LightMagenta => Some((255, 0, 255)),
        Color::LightCyan => Some((0, 255, 255)),
        Color::White => Some((255, 255, 255)),
        Color::Rgb(r, g, b) => Some((r, g, b)),
        _ => None,
    }
}

pub fn get_sized_buffer() -> Vec<Vec<Cell>> {
    let size = if utils::is_mobile() {
        utils::get_screen_size()
    } else {
        utils::get_window_size()
    };
    vec![
        vec![Cell::default(); (size.width * MULTIPLIER) as usize];
        (size.height * MULTIPLIER) as usize
    ]
}

pub struct HdBackend {
    /// Whether the backend has been initialized.
    initialized: Rc<RefCell<bool>>,
    /// Current buffer.
    buffer: Vec<Vec<Cell>>,
    /// Previous buffer.
    prev_buffer: Vec<Vec<Cell>>,
    /// Cells.
    cells: Vec<Element>,
    /// Grid element.
    grid: Element,
    /// Window.
    window: Window,
    /// Document.
    document: Document,
}

impl HdBackend {
    /// Constructs a new [`DomBackend`].
    pub fn new() -> Result<Self, Error> {
        let window = window().ok_or(Error::UnableToRetrieveWindow)?;
        let document = window.document().ok_or(Error::UnableToRetrieveDocument)?;
        let mut backend = Self {
            initialized: Rc::new(RefCell::new(false)),
            buffer: vec![],
            prev_buffer: vec![],
            cells: vec![],
            grid: document.create_element("div")?,
            window,
            document,
        };
        backend.add_on_resize_listener();
        backend.reset_grid()?;
        Ok(backend)
    }

    /// Add a listener to the window resize event.
    fn add_on_resize_listener(&mut self) {
        let initialized = self.initialized.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
            initialized.replace(false);
        });
        self.window
            .set_onresize(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    /// Reset the grid and clear the cells.
    fn reset_grid(&mut self) -> Result<(), Error> {
        self.grid = self.document.create_element("div")?;
        self.grid.set_attribute("id", "grid")?;
        self.cells.clear();
        self.buffer = get_sized_buffer();
        self.prev_buffer = self.buffer.clone();
        Ok(())
    }

    /// Pre-render the content to the screen.
    ///
    /// This function is called from [`flush`] once to render the initial
    /// content to the screen.
    fn prerender(&mut self) -> Result<(), Error> {
        for line in self.buffer.iter() {
            let mut line_cells: Vec<Element> = Vec::new();
            for cell in line.iter() {
                let span = create_span(&self.document, cell)?;
                self.cells.push(span.clone());
                line_cells.push(span);
            }

            // Create a <pre> element for the line
            let pre = self.document.create_element("pre")?;

            // Append all elements (spans and anchors) to the <pre>
            for elem in line_cells {
                pre.append_child(&elem)?;
            }

            // Append the <pre> to the grid
            self.grid.append_child(&pre)?;
        }
        Ok(())
    }

    /// Compare the current buffer to the previous buffer and updates the grid
    /// accordingly.
    fn update_grid(&mut self) -> Result<(), Error> {
        for (y, line) in self.buffer.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell != &self.prev_buffer[y][x] {
                    let elem = self.cells[y * self.buffer[0].len() + x].clone();
                    elem.set_inner_html(cell.symbol());
                    elem.set_attribute("style", &get_cell_style_as_css(cell))?;
                }
            }
        }
        Ok(())
    }
}

impl Backend for HdBackend {
    // Populates the buffer with the given content.
    fn draw<'a, I>(&mut self, content: I) -> IoResult<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        if !*self.initialized.borrow() {
            // Only runs on resize event.
            if let Some(grid) = self.document.get_element_by_id("grid") {
                grid.remove();
                self.reset_grid()?;
            }
        }

        // Update the cells with new content
        for (x, y, cell) in content {
            let y = y as usize;
            let x = x as usize;
            if y < self.buffer.len() {
                let line = &mut self.buffer[y];
                line.extend(
                    std::iter::repeat_with(Cell::default).take(x.saturating_sub(line.len())),
                );
                if x < line.len() {
                    line[x] = cell.clone();
                }
            }
        }
        Ok(())
    }

    /// Flush the content to the screen.
    ///
    /// This function is called after the [`DomBackend::draw`] function to
    /// actually render the content to the screen.
    fn flush(&mut self) -> IoResult<()> {
        if !*self.initialized.borrow() {
            self.initialized.replace(true);
            let body = self.document.body().ok_or(Error::UnableToRetrieveBody)?;
            body.append_child(&self.grid).map_err(Error::from)?;
            self.prerender()?;
            // Set the previous buffer to the current buffer for the first render
            self.prev_buffer = self.buffer.clone();
        }
        // Check if the buffer has changed since the last render and update the grid
        if self.buffer != self.prev_buffer {
            self.update_grid()?;
        }
        self.prev_buffer = self.buffer.clone();
        Ok(())
    }

    fn hide_cursor(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn get_cursor(&mut self) -> IoResult<(u16, u16)> {
        Ok((0, 0))
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> IoResult<()> {
        Ok(())
    }

    fn clear(&mut self) -> IoResult<()> {
        self.buffer = get_sized_buffer();
        Ok(())
    }

    fn size(&self) -> IoResult<Size> {
        Ok(Size::new(
            self.buffer[0].len().saturating_sub(1) as u16,
            self.buffer.len().saturating_sub(1) as u16,
        ))
    }

    fn window_size(&mut self) -> IoResult<WindowSize> {
        unimplemented!()
    }

    fn get_cursor_position(&mut self) -> IoResult<Position> {
        unimplemented!()
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, _: P) -> IoResult<()> {
        unimplemented!()
    }
}
