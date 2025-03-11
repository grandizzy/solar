use solar::{
    interface::{diagnostics::EmittedDiagnostics, Session},
    sema::{
        hir::{Arena, ContractId},
        thread_local::ThreadLocal,
        ParsingContext,
    },
};
use std::path::Path;

#[test]
fn main() -> Result<(), EmittedDiagnostics> {
    let paths = vec![Path::new("src/AnotherCounter.sol")];

    // Create a new session with a buffer emitter.
    // This is required to capture the emitted diagnostics and to return them at the end.
    let sess = Session::builder().with_buffer_emitter(solar::interface::ColorChoice::Auto).build();

    // Enter the context and parse the file.
    // Counter will be parsed, even if not explicitly provided, since it is a dependency.
    let _ = sess.enter_parallel(|| -> solar::interface::Result<()> {
        // Set up the parser.
        let hir_arena = ThreadLocal::<Arena>::new();
        let mut parsing_context = ParsingContext::new(&sess);
        parsing_context.load_files(paths)?;

        if let Some(gcx) = parsing_context.parse_and_lower(&hir_arena)? {
            for contract in gcx.hir().contracts() {
                println!("contract: {}", contract.name);
            }
            let counter_contract = gcx.hir().contract(ContractId::new(0));
            assert_eq!(counter_contract.name.to_string(), "Counter");
            let another_counter_contract = gcx.hir().contract(ContractId::new(1));
            assert_eq!(another_counter_contract.name.to_string(), "AnotherCounter");
        }

        Ok(())
    });

    // Return the emitted diagnostics as a `Result<(), _>`.
    // If any errors were emitted, this returns `Err(_)`, otherwise `Ok(())`.
    sess.emitted_errors().unwrap()
}
