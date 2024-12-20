use log::{trace, warn};

use hubuum_client::FilterOperator;

use crate::commandlist::CommandList;

pub fn bool(_cmdlist: &CommandList, _prefix: &str, _parts: &[String]) -> Vec<String> {
    vec!["true".to_string(), "false".to_string()]
}

pub fn classes(cmdlist: &CommandList, prefix: &str, _parts: &[String]) -> Vec<String> {
    trace!("Autocompleting classes with prefix: {}", prefix);
    let mut cmd = cmdlist.client().classes().find();

    if !prefix.is_empty() {
        cmd = cmd.add_filter(
            "name",
            FilterOperator::StartsWith { is_negated: false },
            prefix,
        );
    }
    match cmd.execute() {
        Ok(classes) => classes.into_iter().map(|c| c.name).collect(),
        Err(_) => {
            warn!("Failed to fetch classes for autocomplete");
            Vec::new()
        }
    }
}

pub fn namespaces(cmdlist: &CommandList, prefix: &str, _parts: &[String]) -> Vec<String> {
    trace!("Autocompleting namespaces with prefix: {}", prefix);
    let mut cmd = cmdlist.client().namespaces().find();

    if !prefix.is_empty() {
        cmd = cmd.add_filter(
            "name",
            FilterOperator::StartsWith { is_negated: false },
            prefix,
        );
    }
    match cmd.execute() {
        Ok(namespaces) => namespaces.into_iter().map(|c| c.name).collect(),
        Err(_) => {
            warn!("Failed to fetch namespaces for autocomplete");
            Vec::new()
        }
    }
}

fn objects_from_class_source(
    cmdlist: &CommandList,
    prefix: &str,
    parts: &[String],
    source: &str,
) -> Vec<String> {
    trace!(
        "Autocompleting objects via source '{}' with prefix: {}",
        source,
        prefix
    );
    let classname = match parts.windows(2).find(|w| w[0] == source) {
        Some(window) => window[1].clone(),
        None => return Vec::new(),
    };

    let class = match cmdlist
        .client()
        .classes()
        .find()
        .add_filter_name_exact(classname)
        .execute_expecting_single_result()
    {
        Ok(ret) => ret,
        Err(_) => {
            warn!("Failed to fetch class from {} autocomplete", source);
            return Vec::new();
        }
    };

    let mut cmd = cmdlist.client().objects(class.id).find();

    if !prefix.is_empty() {
        cmd = cmd.add_filter(
            "name",
            FilterOperator::StartsWith { is_negated: false },
            prefix,
        );
    }

    match cmd.execute() {
        Ok(objects) => objects.into_iter().map(|c| c.name).collect(),
        Err(_) => {
            warn!("Failed to fetch objects for autocomplete");
            Vec::new()
        }
    }
}

pub fn objects_from_class(cmdlist: &CommandList, prefix: &str, parts: &[String]) -> Vec<String> {
    objects_from_class_source(cmdlist, prefix, parts, "--class")
}

pub fn objects_from_class_from(
    cmdlist: &CommandList,
    prefix: &str,
    parts: &[String],
) -> Vec<String> {
    objects_from_class_source(cmdlist, prefix, parts, "--class_from")
}

pub fn objects_from_class_to(cmdlist: &CommandList, prefix: &str, parts: &[String]) -> Vec<String> {
    objects_from_class_source(cmdlist, prefix, parts, "--class_to")
}
