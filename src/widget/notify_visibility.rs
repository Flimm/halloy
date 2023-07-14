use std::cell::RefCell;

use iced::advanced::widget::tree;
use iced::advanced::{layout, overlay, renderer, widget, Clipboard, Layout, Shell, Widget};
pub use iced::keyboard::{KeyCode, Modifiers};
use iced::{event, mouse, window, Event, Length, Rectangle};

use super::{Element, Renderer};
use crate::Theme;

#[derive(Debug, Clone, Copy)]
pub enum When {
    Visible,
    NotVisible,
}

pub fn notify_visibility<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    margin: f32,
    when: When,
    message: Message,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    NotifyVisibility {
        content: content.into(),
        margin,
        when,
        message,
        state: RefCell::default(),
    }
    .into()
}

struct NotifyVisibility<'a, Message> {
    content: Element<'a, Message>,
    margin: f32,
    when: When,
    message: Message,
    state: RefCell<State>,
}

impl<'a, Message> Widget<Message, Renderer> for NotifyVisibility<'a, Message>
where
    Message: Clone,
{
    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport)
    }

    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<widget::Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        self.content.as_widget().diff(tree);
    }

    fn operate(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content
            .as_widget()
            .operate(tree, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let Event::Window(window::Event::RedrawRequested(_)) = &event {
            let mut state = self.state.borrow_mut();

            let is_visible = viewport.expand(self.margin).intersects(&layout.bounds());

            let should_notify = match self.when {
                When::Visible => is_visible,
                When::NotVisible => !is_visible,
            };

            if should_notify && !state.sent {
                shell.publish(self.message.clone());
                state.sent = true;
            }
        }

        self.content.as_widget_mut().on_event(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget_mut().overlay(tree, layout, renderer)
    }
}

#[derive(Default)]
struct State {
    sent: bool,
}

impl<'a, Message> From<NotifyVisibility<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(notify_visibility: NotifyVisibility<'a, Message>) -> Self {
        Element::new(notify_visibility)
    }
}
