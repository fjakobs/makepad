#![allow(unused)]
use {
    crate::{
        makepad_platform::*,
        button_logic::*,
        frame_component::*,
    }
};

live_register!{
    use makepad_platform::shader::std::*;
    use crate::theme::*;
    
    TextInput: {{TextInput}} {
        
        label_text: {
            instance hover: 0.0
            instance focus: 0.0
            text_style: FONT_CODE {}
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        #9,
                        #f,
                        self.hover
                    ),
                    #0,
                    self.focus
                )
            }
        }
        
        bg_quad: {
            instance hover: 0.0
            instance focus: 0.0
            
            const BORDER_RADIUS: 2.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    BORDER_RADIUS
                )
                sdf.fill(#0000)
                return sdf.result
            }
        }
        
        walk: {
            width: Size::Fit,
            height: Size::Fill,
            margin: {left: 1.0, right: 1.0, top: 2.0, bottom: 2.0},
        }
        
        layout: {
            align: {y: 0.5}
            padding: {left: 4.0, top: 0.0, right: 4.0, bottom: 2.0}
        }
        
        state: {
            hover = {
                default:off
                off = {
                    from: {all: Play::Forward {duration: 0.1}}
                    apply: {
                        bg_quad: {hover: 0.0}
                        label_text: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Play::Snap}
                    apply: {
                        bg_quad: {hover: 1.0}
                        label_text: {hover: 1.0}
                    }
                }
            }
            focus = {
                default:off
                off = {
                    from: {all: Play::Forward {duration: 0.1}}
                    apply: {
                        bg_quad: {focus: 0.0}
                        label_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Play::Snap}
                    apply: {
                        bg_quad: {focus: 1.0}
                        label_text: {focus: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
#[live_register(register_as_frame_component!(TextInput))]
pub struct TextInput {
    state: State,
    
    bg_quad: DrawQuad,
    label_text: DrawText,
    
    walk: Walk,
    layout: Layout,
    
    pub value: String
}

impl FrameComponent for TextInput {
    fn handle_component_event(&mut self, cx: &mut Cx, event: &mut Event, self_id: LiveId) -> FrameComponentActionRef {
        self.handle_event(cx, event).into()
    }
    
    fn get_walk(&self) -> Walk {
        self.walk
    }
    
    fn draw_component(&mut self, cx: &mut Cx2d, walk: Walk) -> Result<(), LiveId> {
        self.draw_walk(cx, walk);
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, FrameComponentAction)]
pub enum TextInputAction {
    None
}

impl TextInput {
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &mut Event) -> TextInputAction {
        self.state_handle_event(cx, event);
        TextInputAction::None
        /*
        self.animator_handle_event(cx, event);
        let res = self.button_logic.handle_event(cx, event, self.bg_quad.draw_vars.area);
        
        match res.state {
            ButtonState::Pressed => self.animate_to(cx, self.pressed_state),
            ButtonState::Default => self.animate_to(cx, self.default_state),
            ButtonState::Hover => self.animate_to(cx, self.hover_state),
            _ => ()
        };
        res.action*/
    }
    
    pub fn draw_label(&mut self, cx: &mut Cx2d, label: &str) {
        self.bg_quad.begin(cx, self.walk, self.layout);
        self.label_text.draw_walk(cx, Walk::default(), label);
        self.bg_quad.end(cx);
    }
    
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.bg_quad.begin(cx, walk, self.layout);
        self.label_text.draw_walk(cx, Walk::default(), &self.value);
        self.bg_quad.end(cx);
    }
}
