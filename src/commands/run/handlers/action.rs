use crate::cfg::Cfg;
use crate::commands::run::actions::act;
use crate::commands::run::processes::Process;
use crate::util::log::LogData;
use std::collections::HashSet;

pub fn handle_action(
    cfg: &Cfg,
    proc: &mut Process,
    log_data: &LogData,
    exec_actions: &mut Vec<String>,
) {
    let mut action_set = HashSet::new();

    if !exec_actions.contains(&"silence".to_string()) {
        action_set.insert("logger".to_string());
    }

    exec_actions.retain(|x| action_set.insert(x.clone()));

    for action in action_set {
        act(cfg, proc, log_data, &action[..]);
    }
}
