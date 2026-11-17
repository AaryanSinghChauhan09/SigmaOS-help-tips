//! Integration tests for SigmaOS-mint-help-tips.

use sigmaos_mint_help_tips_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
