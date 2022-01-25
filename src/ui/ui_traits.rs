pub trait CUI {
    fn new(path: &str) -> Self;
    fn save(&self) -> bool;
}

pub trait QUI {
    fn list_all_sessions(&self);
    fn connect_to_session_by_index(&self, index: usize);
    fn remove_session_group_by_name(&mut self, name: &str);
}

pub trait FUI {
    fn main_menu(&mut self);
    fn add_menu(&mut self);
    fn remove_menu(&mut self);
}
