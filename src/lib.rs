#[macro_export(local_inner_macros)]
macro_rules! k8s_actor_addr {
    () => {
        "/k8s"
    };
}

#[macro_export(local_inner_macros)]
macro_rules! state_actor_addr {
    () => {
        "/state"
    };
}

#[macro_export(local_inner_macros)]
macro_rules! llm_actor_addr {
    () => {
        "/llm"
    };
}

#[macro_export(local_inner_macros)]
macro_rules! runtime_actor_addr {
    () => {
        "/runtime"
    };
}

#[macro_export(local_inner_macros)]
macro_rules! log_actor_addr {
    () => {
        "/log"
    };
}
