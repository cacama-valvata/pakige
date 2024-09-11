use crate::{Pakige, PakigeParseError, VerOp};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;
use deb_version7::DebVersion;
use regex::Regex;

mod setters;
use setters::{set_package, set_source, set_version, set_section, set_priority,
              set_architecture, set_essential, set_depends, set_recommends,
              set_suggests, set_enhances, set_pre_depends, set_breaks, set_conflicts,
              set_provides, set_replaces, set_installed_size, set_maintainer,
              set_description, set_homepage, set_built_using, set_multi_arch};

pub struct BinaryDeb
{
    pub package: String, /* Mandatory */
    pub source: Option<String>,
    pub version: DebVersion, /* Mandatory */
    pub section: Option<String>, /* Recommended */
    pub priority: Option<String>, /* Recommended */
    pub architecture: String, /* Mandatory */
    pub essential: bool,
    pub depends: Option<DependsPackageList>,
    pub recommends: Option<DependsPackageList>,
    pub suggests: Option<DependsPackageList>,
    pub enhances: Option<DependsPackageList>,
    pub pre_depends: Option<DependsPackageList>,
    pub breaks: Option<ProvidesPackageList>,
    pub conflicts: Option<ProvidesPackageList>,
    pub provides: Option<ProvidesPackageList>,
    pub replaces: Option<ProvidesPackageList>,
    pub installed_size: Option<u64>,
    pub maintainer: String, /* Mandatory */
    pub description: String, /* Mandatory */
    pub homepage: Option<String>,
    pub built_using: Option<ProvidesPackageList>,
    pub multi_arch: MultiArch,
    pub all_fields: Fields
}

fn str_to_table (data: &str) -> Result<Fields,PakigeParseError>
{
    // https://man7.org/linux/man-pages/man5/deb822.5.html
    // The field name
    // is composed of US-ASCII characters excluding control characters,
    // space, and colon (i.e., characters in the ranges U+0021 ‘!’
    // through U+0039 ‘9’, and U+003B ‘;’ through U+007E ‘~’,
    // inclusive).  Field names must not begin with the comment
    // character (U+0023 ‘#’), nor with the hyphen character (U+002D
    // ‘-’).
    let normal_line = Regex::new(r"^[[:space:]]*([[!-9--#---][;-~]][[!-9][;-~]]*)[[:space:]]*:[[:space:]]*(.*)[[:space:]]*$").unwrap();
    let continuation = Regex::new(r"^( .*)$").unwrap();

    let mut lines: VecDeque<&str> = data.trim().lines().collect();
    let mut fields: Fields = HashMap::new();

    while let Some(line) = lines.pop_front()
    {
        /* There should never be continuation lines in this outer loop */
        /* The first loop starts with the first line of the stanza, which can not be a continuation line */
        // Look for normal line, capture key-value pairs
        if let Some(captures) = normal_line.captures(line)
        {
            let key = captures.get(1)
                .ok_or(PakigeParseError::InvalidFormat)?
                .as_str()
                .to_lowercase(); // Note: "Field names are not case-sensitive." RFC 822
            let value = captures.get(2)
                .ok_or(PakigeParseError::InvalidFormat)?
                .as_str();

            // Check for duplicate fields
            if fields.contains_key(&key)
            {
                return Err(PakigeParseError::DuplicateField);
            }

            // We may need to append further 
            let mut field_data = String::from(value);

            // Look ahead for continuation lines
            while let Some(next_line) = lines.pop_front()
            {
                if let Some(capture_append) = continuation.captures(next_line)
                {
                    let to_append = capture_append.get(1)
                    .ok_or(PakigeParseError::InvalidFormat)?
                    .as_str();

                    field_data.push('\n'); // still preserves the double space that indicates to not wrap
                    field_data.push_str(to_append);
                }
                /* Break/Base case */
                else
                {
                    lines.push_front(next_line); // put it back to be dealt with in the outer loop
                    break;
                }
            }

            // Insert into Hashmap
            fields.insert(key, field_data);
        }
        // Else, not valid Debian Control syntax
        else
        {
            return Err(PakigeParseError::InvalidFormat);
        }
    }

    if fields.is_empty()
    {
        return Err(PakigeParseError::EmptyInput);
    }

    return Ok(fields);
}

type Fields = HashMap<String, String>;

impl FromStr for BinaryDeb
{
    type Err = PakigeParseError;

    fn from_str (data: &str) -> Result<Self, Self::Err>
    {
        let mut fields = str_to_table(data)?;

        let mut deb = BinaryDeb {
            //all_fields: fields,
            package: set_package (&fields)?.ok_or (PakigeParseError::MissingMandatoryField)?, /* Mandatory */
            source: set_source (&fields)?,
            version: set_version (&fields)?.ok_or (PakigeParseError::MissingMandatoryField)?, /* Mandatory */
            section: set_section (&fields)?, /* Recommended */
            priority: set_priority (&fields)?, /* Recommended */
            architecture: set_architecture (&fields)?.ok_or (PakigeParseError::MissingMandatoryField)?, /* Mandatory */
            essential: set_essential (&fields)?.unwrap_or (false), // Has default value
            depends: set_depends (&fields)?,
            recommends: set_recommends (&fields)?,
            suggests: set_suggests (&fields)?,
            enhances: set_enhances (&fields)?,
            pre_depends: set_pre_depends (&fields)?,
            breaks: set_breaks (&fields)?,
            conflicts: set_conflicts (&fields)?,
            provides: set_provides (&fields)?,
            replaces: set_replaces (&fields)?,
            installed_size: set_installed_size (&fields)?,
            maintainer: set_maintainer (&fields)?.ok_or (PakigeParseError::MissingMandatoryField)?, /* Mandatory */
            description: set_description (&fields)?.ok_or (PakigeParseError::MissingMandatoryField)?, /* Mandatory */
            homepage: set_homepage (&fields)?,
            built_using: set_built_using (&fields)?,
            multi_arch: set_multi_arch (&fields)?.unwrap_or (MultiArch::No), // Has default value
            all_fields: fields,
        };

        Ok(deb)
    }
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

pub struct PackageIndex(Vec<(BinaryDeb, BinaryIndexFields)>); // TODO: would rather this be keyword indexed

// impl From<&str> for PackageIndex
// {
//     fn from (data: &str) -> Self
//     {
//         //
//     }
// }

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
    version_string: DebVersion
}

/* Depends, Pre-Depends, Recommends, Suggests, Enhances */
pub struct DependsPackageList(Vec<Vec<PackageRef>>);
// Inner Vec are pipe expressions (groups): `pkg | pkg | pkg`
// Outer Vec are comma expressions between groups: pkg, pkg

/* Breaks, Conflicts, Replaces, Provides */
pub struct ProvidesPackageList(Vec<PackageRef>);
// These types only use comma expressions between packages

#[derive(Clone, Copy)]
pub enum MultiArch
{
    No,
    Same,
    Foreign,
    Allowed
}

impl Default for MultiArch
{
    fn default() -> Self
    {
        MultiArch::No
    }
}

// pub fn parse_packages_file (data: &str) -> Vec<BinaryDeb>
// {
//     //
// }



// for source debs
/*
let comment_line = Regex::new(r"^#.*$").unwrap();


// Look for comment line
else if comment_line.is_match(line)
{
continue;
}
*/