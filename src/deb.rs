use crate::{Pakige, PakigeParseError, VerOp};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;
use regex::Regex;

#[derive(Default)]
pub struct BinaryDeb
{
    package: String, /* Mandatory */
    source: Option<String>,
    version: String, /* Mandatory */
    section: Option<String>, /* Recommended */
    priority: Option<String>, /* Recommended */
    architecture: String, /* Mandatory */
    essential: bool,
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
    multi_arch: MultiArch,
    index_fields: Option<BinaryIndexFields>,
    all_fields: Option<HashMap<String, String>>
}

impl FromStr for BinaryDeb
{
    type Err = PakigeParseError;

    // Whitespace can be basically anywhere, but
    // There is also no requirement to separate keys and values with spaces

    fn from_str (data: &str) -> Result<Self, Self::Err>
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
        let mut fields: HashMap<String, String> = HashMap::new();
        // let mut newlines: VecDeque<&str> = VecDeque::new();
        // // reduce multi-line value into one &str
        while let Some(line) = lines.pop_front()
        {
            /* There should never be continuation lines in this outer loop */
            /* The first loop starts with the first line of the stanza, which can not be a continuation line */
            // Look for normal line, capture key-value pairs
            if let Some(captures) = normal_line.captures(line)
            {
                let key = captures.get(1)
                    .ok_or(PakigeParseError::InvalidFormat)?
                    .as_str();
                let value = captures.get(2)
                    .ok_or(PakigeParseError::InvalidFormat)?
                    .as_str();

                // Check for duplicate fields
                if fields.contains_key(key)
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

                // insert into Hashmap
                fields.insert(String::from(key.to_lowercase()), field_data);
            }
            // Else, not valid Debian Control syntax
            else
            {
                return Err(PakigeParseError::InvalidFormat);
            }
        }

        for (key, value) in &fields
        {
            println!("\"{}\" : \"{}\"", key, value);
        }

        // ... TODO: to turn into a struct

        Ok(Default::default())
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

pub struct PackageIndex(Vec<BinaryDeb>);

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
    version_string: String
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