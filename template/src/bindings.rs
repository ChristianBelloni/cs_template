use interoptopus::InventoryBuilder;
use interoptopus::Inventory;
use interoptopus::Interop;
use interoptopus::util::NamespaceMappings;
use interoptopus::Error;

const CLASS_NAME: &str = "{{interop_class}}";
const NAMESPACE: &str = "{{namespace}}";

pub fn ffi_inventory() -> Inventory {
    println!("building bindings for {}.{}", CLASS_NAME, NAMESPACE);
    InventoryBuilder::new()
        .inventory()
}

pub fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus_backend_csharp::overloads::{DotNet};

    let config = Config {
        dll_name: CLASS_NAME.to_string(),
        namespace_mappings: NamespaceMappings::new(NAMESPACE),
        ..Config::default()
    };

    Generator::new(config, crate::bindings::ffi_inventory())
        .add_overload_writer(DotNet::new())
        //.add_overload_writer(Unity::new())
        .write_file("/csharp/Interop.cs")?;

    Ok(())
}

#[cfg(test)]
pub mod c_sharp{
    #[test]
    pub fn c_sharp(){
        crate::bindings::ffi_inventory();
    }
}