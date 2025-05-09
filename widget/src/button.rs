use strelka_core::{theme::StyleConverter, Theme};

use iced::{
    advanced::{
        graphics::geometry::{self, Frame},
        layout, mouse,
        renderer::{self},
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    touch,
    widget::{
        canvas::{self, path::Builder, Fill},
        text::IntoFragment,
    },
    window, Alignment, Element, Event, Length, Padding, Point, Rectangle, Renderer, Size, Vector,
};
use theming::{stylesheet::ButtonStyle, Radius};
use theming::{Border, Color, Font, Margin};

pub fn primary_button<'a, Message>(text: impl IntoFragment<'a>) -> Button<'a, Message, Renderer> {
    Button::new(
        iced::widget::text(text)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
}

pub fn secondary_button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> iced::widget::Button<'a, Message, Theme> {
    iced::widget::Button::new(content).style(strelka_core::iced::button::secondary)
}

pub fn text_button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> iced::widget::Button<'a, Message, Theme> {
    iced::widget::Button::new(content).style(strelka_core::iced::button::secondary)
}

#[derive(Debug, Clone)]
pub struct Style {
    pub background: Color,
    pub margin: Margin,
    pub border: Border,
    pub font: Font,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

pub struct Button<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    padding: Padding,
    height: Length,
    width: Length,
    on_press: Option<Message>,
    status: Option<Status>,
}

impl<'a, Message, Renderer: iced::advanced::Renderer> Button<'a, Message, Renderer> {
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let content_size = content.as_widget().size_hint();
        Self {
            content,
            padding: Padding::new(5.0).left(10.0).right(10.0),
            height: content_size.height.fluid(),
            width: content_size.width.fluid(),
            on_press: None,
            status: None,
        }
    }

    pub fn height(mut self, value: impl Into<Length>) -> Self {
        self.height = value.into();
        self
    }

    pub fn width(mut self, value: impl Into<Length>) -> Self {
        self.width = value.into();
        self
    }

    pub fn padding(mut self, value: impl Into<Padding>) -> Self {
        self.padding = value.into();
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for Button<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(limits, self.width, self.height, self.padding, |limits| {
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        })
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = &self.on_press {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press.clone());
                        }

                        shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        let current_status = if self.on_press.is_none() {
            Status::Disabled
        } else if cursor.is_over(layout.bounds()) {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style_: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let is_mouse_over = cursor.is_over(layout.bounds());

        let status = if is_mouse_over {
            let state = tree.state.downcast_ref::<State>();
            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style = match status {
            Status::Active => {
                let style: ButtonStyle = ButtonStyle::from_theme(theme, "button.active");
                Style {
                    background: style.background,
                    border: Border::with_radius(style.border_radius),
                    margin: Margin::new(0.0),
                    font: Font::SANS_SERIF,
                }
            }
            Status::Hovered => {
                let style: ButtonStyle = ButtonStyle::from_theme(theme, "button.hover");
                Style {
                    background: style.background,
                    border: Border::with_radius(style.border_radius),
                    margin: Margin::new(0.0),
                    font: Font::SANS_SERIF,
                }
            }
            Status::Pressed => {
                let style: ButtonStyle = ButtonStyle::from_theme(theme, "button.selected");
                Style {
                    background: style.background,
                    border: Border::with_radius(style.border_radius),
                    margin: Margin::new(0.0),
                    font: Font::SANS_SERIF,
                }
            }
            Status::Disabled => {
                let style: ButtonStyle = ButtonStyle::from_theme(theme, "button.disabled");
                Style {
                    background: style.background,
                    border: Border::with_radius(style.border_radius),
                    margin: Margin::new(0.0),
                    font: Font::SANS_SERIF,
                }
            }
        };

        let width = layout.bounds().width;
        let height = layout.bounds().height;

        let radius = Radius {
            top_left: style
                .border
                .radius
                .top_left
                .min(width / 2.0)
                .min(height / 2.0),
            top_right: style
                .border
                .radius
                .top_right
                .min(width / 2.0)
                .min(height / 2.0),
            bottom_right: style
                .border
                .radius
                .bottom_right
                .min(width / 2.0)
                .min(height / 2.0),
            bottom_left: style
                .border
                .radius
                .bottom_left
                .min(width / 2.0)
                .min(height / 2.0),
        };

        let widget_bounds = Size::new(width, height);

        // Calculate the content area bounds accounting for margins
        let content_origin = Point::new(style.margin.left, style.margin.top);

        let content_bounds = Rectangle::new(
            content_origin,
            Size::new(
                width - style.margin.left - style.margin.right,
                height - style.margin.top - style.margin.bottom,
            ),
        );

        // Draw the button shape
        let mut frame = Frame::new(renderer, widget_bounds);
        let mut builder = Builder::new();

        // Top line
        builder.move_to(Point::new(
            content_bounds.x + radius.top_left,
            content_bounds.y,
        ));
        builder.line_to(Point::new(
            content_bounds.x + content_bounds.width - radius.top_right,
            content_bounds.y,
        ));

        // Top right arc
        builder.arc_to(
            Point::new(content_bounds.x + content_bounds.width, content_bounds.y),
            Point::new(
                content_bounds.x + content_bounds.width,
                content_bounds.y + content_bounds.height - radius.bottom_right.abs(),
            ),
            radius.top_right.abs(),
        );

        // Bottom right arc
        builder.arc_to(
            Point::new(
                content_bounds.x + content_bounds.width,
                content_bounds.y + content_bounds.height,
            ),
            Point::new(
                content_bounds.x + content_bounds.width - radius.bottom_right,
                content_bounds.y + content_bounds.height,
            ),
            radius.bottom_right.abs(),
        );

        // Bottom line
        builder.line_to(Point::new(
            content_bounds.x + radius.bottom_left,
            content_bounds.y + content_bounds.height,
        ));

        // Bottom left arc
        builder.arc_to(
            Point::new(content_bounds.x, content_bounds.y + content_bounds.height),
            Point::new(
                content_bounds.x,
                content_bounds.y + content_bounds.height - radius.bottom_left.abs(),
            ),
            radius.bottom_left.abs(),
        );

        // Top left arc
        builder.arc_to(
            Point::new(content_bounds.x, content_bounds.y),
            Point::new(content_bounds.x + radius.top_left, content_bounds.y),
            radius.top_left.abs(),
        );

        let path = builder.build();

        // Draw background
        frame.fill(
            &path,
            Fill {
                style: canvas::Style::Solid(style.background.into()),
                ..Default::default()
            },
        );

        // Draw border
        // frame.stroke(
        //     &path,
        //     Stroke {
        //         style: canvas::Style::Solid(style.border.color.into()),
        //         width: border_width,
        //         ..Default::default()
        //     },
        // );

        let content_layout = layout.children().next().unwrap();

        // Draw the frame with proper translation
        let geometry = frame.into_geometry();
        renderer.with_translation(
            Vector::new(
                layout.bounds().x - content_bounds.x,
                layout.bounds().y - content_bounds.y,
            ),
            |renderer| {
                renderer.draw_geometry(geometry);
            },
        );

        // Draw the content
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style_.text_color,
            },
            content_layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<Button<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + 'a,
{
    fn from(button: Button<'a, Message, Renderer>) -> Self {
        Self::new(button)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Hovered,
    Pressed,
    Disabled,
}
