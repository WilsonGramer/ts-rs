use quote::quote;
use syn::Result;

use crate::{attr::StructAttr, deps::Dependencies, DerivedTS};

pub(crate) fn empty_object(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;

    Ok(DerivedTS {
        inline: quote!("Record<string, never>".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::default(),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
    })
}

pub(crate) fn empty_array(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;

    Ok(DerivedTS {
        inline: quote!("never[]".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::default(),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
    })
}

pub(crate) fn null(attr: &StructAttr, name: &str) -> Result<DerivedTS> {
    check_attributes(attr)?;

    Ok(DerivedTS {
        inline: quote!("null".to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::default(),
        export: attr.export,
        export_to: attr.export_to.clone(),
        ts_name: name.to_owned(),
        concrete: attr.concrete.clone(),
    })
}

fn check_attributes(attr: &StructAttr) -> Result<()> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to unit structs");
    }

    Ok(())
}
