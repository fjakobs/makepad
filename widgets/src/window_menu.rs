// a window menu implementation
use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
};
use std::collections::HashMap;

live_design!{
    WindowMenuBase = {{WindowMenu}}{
    }
}

#[derive(Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum WindowMenuItem {
    #[pick {items: vec![]}]
    Main{items:Vec<LiveId>},
    #[live {name:"Unknown".to_string(), shift: false, key:KeyCode::Unknown, enabled:true }]
    Item{
        name: String,
        shift: bool,
        key: KeyCode,
        enabled: bool
    },
    #[live {name:"Unknown".to_string(), items:vec![] }]
    Sub{
        name:String,
        items:Vec<LiveId>
    },
    #[live]
    Line
}

#[derive(Live)]
pub struct WindowMenu{
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[rust] menu_items: HashMap<LiveId, WindowMenuItem>,
}

#[derive(Clone, WidgetAction)]
pub enum WindowMenuAction {
    Command(LiveId),
    None
}

impl LiveHook for WindowMenu {
    fn apply_value_instance(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) -> usize {
        let id = nodes[index].id;
        match from {
            ApplyFrom::NewFromDoc {..} | ApplyFrom::UpdateFromDoc {..} => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    if nodes[index].value.is_enum() {
                        let mut dock_item = WindowMenuItem::new(cx);
                        let index = dock_item.apply(cx, from, index, nodes);
                        self.menu_items.insert(id, dock_item);
                        return index;
                    }
                }
                else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => ()
        }
        nodes.skip_node(index)
    }
    
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // lets translate the menu into a macos menu
        #[cfg(target_os="macos")]{
            // alright lets fetch this thing
            fn recur_menu(command:LiveId,menu_items:&HashMap<LiveId, WindowMenuItem>)->MacosMenu{
                
                if let Some(item) = menu_items.get(&command){
                    match item.clone(){
                        WindowMenuItem::Main{items}=>{
                            let mut out = Vec::new();
                            for item in items{
                                out.push(recur_menu(item, menu_items));
                            }
                            return MacosMenu::Main{items:out}
                        }
                        WindowMenuItem::Item{name, shift, key, enabled}=>{
                            return MacosMenu::Item{
                                command,
                                name,
                                shift,
                                key,
                                enabled
                            }
                        }
                        WindowMenuItem::Sub{name, items}=>{
                            let mut out = Vec::new();
                            for item in items{
                                out.push(recur_menu(item, menu_items));
                            }
                            return MacosMenu::Sub{name, items:out}
                        }
                        WindowMenuItem::Line=>{
                            return MacosMenu::Line
                        }
                    }
                }
                else{
                    log!("Menu cannot find item {}", command);
                    MacosMenu::Line
                }
            }
            let menu = recur_menu(live_id!(main), &self.menu_items);
            _cx.update_macos_menu(menu)
        }
    }
    
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, WindowMenu);
    }
}


impl Widget for WindowMenu {
    fn redraw(&mut self, _cx: &mut Cx) {
    }
    
    fn handle_widget_event_with(&mut self, cx: &mut Cx, event: &Event, _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)) {
        match event{
            Event::MacosMenuCommand(item)=>{
                if *item == live_id!(quit){
                    cx.quit();
                }
            }
            _=>()
        }
    }
    
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        return Walk::fixed(0.0,0.0);
    }
    
    fn draw_walk_widget(&mut self, _cx: &mut Cx2d, _walk: Walk) -> WidgetDraw {
        WidgetDraw::done()
    }
}


#[derive(Clone, Debug, PartialEq, WidgetRef)]
pub struct WindowMenuRef(WidgetRef);

impl WindowMenuRef {
    pub fn command(&self) -> Option<LiveId> {
        if let Some(mut _dock) = self.borrow_mut() {
          
        }
        None
    }
}
    
