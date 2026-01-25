mod error;
mod exit;
mod locations;
mod theme;
use super::*;
impl NStreamer {
    pub fn add_user_interaction(
        &mut self,
        interaction: DynView<Self, Message>,
        priority: Priority,
    ) {
        self.user_interactions.push(UserInteraction {
            view: interaction,
            priority,
        });
        self.user_interactions.sort();
    }
    pub fn get_top_user_interaction(&self) -> Option<&DynView<Self, Message>> {
        self.user_interactions.iter().last().map(|u| &u.view)
    }
    pub fn close_user_interaction(&mut self) {
        self.user_interactions
            .remove(self.user_interactions.len() - 1);
    }
}

pub struct UserInteraction {
    view: DynView<NStreamer, Message>,
    priority: Priority,
}

impl UserInteraction {
    pub fn priority(&self) -> Priority {
        self.priority
    }
}

impl PartialEq for UserInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}
impl Eq for UserInteraction {}

impl PartialOrd for UserInteraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UserInteraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[allow(unused)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    Exit = 16,
    Error = 8,
    Warn = 4,
    Task = 2,
    Info = 1,
}
