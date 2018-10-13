use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use super::*;

pub trait RouteServiceTrait {
    fn get_route(&self, instance: &Instance) -> Result<Option<Vec<Mission>>>;
}

pub struct RouteServiceImpl {
    pub delivery_service: Rc<DeliveryServiceTrait>,
    pub one_step_flow_cache: Rc<OneStepFlowCacheTrait>,
}

impl RouteServiceTrait for RouteServiceImpl {
    fn get_route(&self, instance: &Instance) -> Result<Option<Vec<Mission>>> {
        if let Ok(Some(relations)) = self.one_step_flow_cache.get(&instance.thing) {
            // no relations
            if relations.len() == 0 {
                return Ok(None);
            }
            let rtn = Self::filter_relations(instance, relations);
            Ok(rtn)
        } else {
            Ok(None)
        }
    }
}

impl RouteServiceImpl {
    fn filter_relations(instance: &Instance, maps: Vec<OneStepFlow>) -> Option<Vec<Mission>> {
//        debug!("filter relations for instance: {:?}", instance);
        let mut rtn: Vec<Mission> = Vec::new();
        for m in maps {
            if !m.selector.is_none() {
                let selector = &m.selector.clone().unwrap();
                if !Self::context_check(&instance.data.context, selector) {
                    continue;
                }
                if !Self::status_check(&instance.data.status, selector) {
                    continue;
                }
            }
            let t = Mission {
                to: m.to.clone(),
                executor: m.executor,
                last_status_demand: {
                    match m.selector {
                        None => None,
                        Some(demand) => {
                            let ld = LastStatusDemand {
                                target_status_include: demand.target_status_include,
                                target_status_exclude: demand.target_status_exclude,
                            };
                            Some(ld)
                        }
                    }
                },
                weight: m.weight,
            };
            rtn.push(t);
        }
        match rtn.len() {
            x  if x > 0 => {
                return Some(rtn);
            }
            _ => return None
        }
    }

    fn context_check(contexts: &HashMap<String, String>, selector: &Selector) -> bool {
        for exclude in &selector.context_exclude {
            if contexts.contains_key(exclude) {
                return false;
            }
        }
        for include in &selector.context_include {
            if !contexts.contains_key(include) {
                return false;
            }
        }
        true
    }

    fn status_check(status: &HashSet<String>, selector: &Selector) -> bool {
        for exclude in &selector.source_status_exclude {
            if status.contains(exclude) {
                return false;
            }
        }
        for include in &selector.source_status_include {
            if !status.contains(include) {
                return false;
            }
        }
        true
    }
}

