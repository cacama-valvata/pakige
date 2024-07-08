pub mod deb
{
    use crate::VerOp;
    use std::collections::HashMap;

    pub struct BinaryDeb
    {
        package: String, /* Mandatory */
        source: Option<String>,
        version: String, /* Mandatory */
        section: Option<String>, /* Recommended */
        priority: Option<String>, /* Recommended */
        architecture: String, /* Mandatory */
        essential: Option<bool>,
        depends: Option<DependsPackageList>,
        recommends: Option<DependsPackageList>,
        suggests: Option<DependsPackageList>,
        enhances: Option<DependsPackageList>,
        pre_depends: Option<DependsPackageList>,
        breaks: Option<ProvidesPackageList>,
        conflicts: Option<ProvidesPackageList>,
        provides: Option<ProvidesPackageList>,
        replaces: Option<ProvidesPackageList>,
        installed_size: Option<u64>,
        maintainer: String, /* Mandatory */
        description: String, /* Mandatory */
        homepage: Option<String>,
        built_using: Option<ProvidesPackageList>,
        multi_arch: Option<MultiArch>,
        index_fields: Option<BinaryIndexFields>,
        other_fields: Option<HashMap<String, String>>
    }

    pub struct BinaryIndexFields
    {
        filename: String, /* Mandatory */
        size: u64, /* Mandatory */
        md5sum: Option<String>, /* Recommended */
        sha1: Option<String>, /* Recommended */
        sha256: Option<String>, /* Recommended */
        sha512: Option<String>, /* Recommended */
        desc_md5: Option<String>
    }

    //pub mod architectures; //TODO

    pub struct PackageRef
    {
        package: String,
        architecture: Option<String>,
        version: Option<VersionRef>
    }

    pub struct VersionRef
    {
        operation: VerOp,
        version_string: String
    }

    /* Depends, Pre-Depends, Recommends, Suggests, Enhances */
    pub type DependsPackageList = Vec<Vec<PackageRef>>;
    // Inner Vec are pipe expressions (groups): `pkg | pkg | pkg`
    // Outer Vec are comma expressions between groups: pkg, pkg

    /* Breaks, Conflicts, Replaces, Provides */
    pub type ProvidesPackageList = Vec<PackageRef>;
    // These types only use comma expressions between packages

    pub enum MultiArch
    {
        No,
        Same,
        Foreign,
        Allowed
    }
}