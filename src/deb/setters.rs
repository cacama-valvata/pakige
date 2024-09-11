use std::{collections::HashMap, ops::Mul};
use crate::PakigeParseError;
use super::{BinaryDeb, DependsPackageList, Fields, MultiArch, PackageRef, ProvidesPackageList};
use regex::Regex;
use deb_version7::DebVersion;
use std::str::FromStr;


static PACKAGENAME_RULES: &str = r"[[:lower:][:digit:]][[:lower:][:digit:][+-.]]+";
//static ARCHITECTURE_RULES: &str = r"";


pub fn set_package (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "package";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };

    let name_rules = Regex::new(PACKAGENAME_RULES).unwrap();
    if name_rules.is_match(value)
    {
        return Ok(Some(value.clone()));
    }
    return Err(PakigeParseError::InvalidValue);
}

pub fn set_source (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "source";

    let value = match fields.get(key)
    {
        Some(value) => value.clone(),
        None => return Ok(None)
    };

    let name_rules = Regex::new(PACKAGENAME_RULES).unwrap();
    if name_rules.is_match(&value)
    {
        return Ok(Some(value));
    }
    return Err(PakigeParseError::InvalidValue);
}

//TODO: create structs, validate syntax of version string
pub fn set_version (fields: &Fields) -> Result<Option<DebVersion>, PakigeParseError>
{
    let key = "version";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };

    let parsed_value = DebVersion::from_str (value)?;

    return Ok(Some(parsed_value));
}

// TODO: reflect the official list of sections
pub fn set_section (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "section";

    let value = match fields.get(key)
    {
        Some(value) => value.clone(),
        None => return Ok(None)
    };
    return Ok(Some(value));
}

// TODO: reflect the official list of priorities
pub fn set_priority (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "priority";

    let value = match fields.get(key)
    {
        Some(value) => value.clone(),
        None => String::from("optional")
    };
    return Ok(Some(value));
}

// TODO: reflect the official list of architectures, or at least syntax
pub fn set_architecture (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "architecture";

    let value = match fields.get(key)
    {
        Some(value) => value.clone(),
        None => return Ok(None)
    };
    return Ok(Some(value))
}

pub fn set_essential (fields: &Fields) -> Result<Option<bool>, PakigeParseError>
{
    let key = "essential";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };

    return match value.as_str()
    {
        // TODO: could these be case-insensitive?
        "yes" => Ok(Some(true)),
        "no" => Ok(Some(false)),
        _ => Err(PakigeParseError::InvalidValue)
    };
}

/* DependsPackageList */
pub fn set_depends (fields: &Fields) -> Result<Option<DependsPackageList>, PakigeParseError>
{
    let key = "depends";
    return Ok(None);
}

/* DependsPackageList */
pub fn set_recommends (fields: &Fields) -> Result<Option<DependsPackageList>, PakigeParseError>
{
    let key = "recommends";
    return Ok(None);
}

/* DependsPackageList */
pub fn set_suggests (fields: &Fields) -> Result<Option<DependsPackageList>, PakigeParseError>
{
    let key = "suggests";
    return Ok(None);
}

/* DependsPackageList */
pub fn set_enhances (fields: &Fields) -> Result<Option<DependsPackageList>, PakigeParseError>
{
    let key = "enhances";
    return Ok(None);
}

/* DependsPackageList */
pub fn set_pre_depends (fields: &Fields) -> Result<Option<DependsPackageList>, PakigeParseError>
{
    let key = "pre-depends";
    return Ok(None);
}

/* ProvidesPackageList */
pub fn set_breaks (fields: &Fields) -> Result<Option<ProvidesPackageList>, PakigeParseError>
{
    let key = "breaks";
    return Ok(None);
}

/* ProvidesPackageList */
pub fn set_conflicts (fields: &Fields) -> Result<Option<ProvidesPackageList>, PakigeParseError>
{
    let key = "conflicts";
    return Ok(None);
}

/* ProvidesPackageList */
pub fn set_provides (fields: &Fields) -> Result<Option<ProvidesPackageList>, PakigeParseError>
{
    let key = "provides";
    return Ok(None);
}

/* ProvidesPackageList */
pub fn set_replaces (fields: &Fields) -> Result<Option<ProvidesPackageList>, PakigeParseError>
{
    let key = "replaces";
    return Ok(None);
}

pub fn set_installed_size (fields: &Fields) -> Result<Option<u64>, PakigeParseError>
{
    let key = "installed-size";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };

    return match value.parse::<u64>()
    {
        Ok(size) => Ok(Some(size)),
        Err(e) => Err(PakigeParseError::InvalidValue)
    };
}


pub fn set_maintainer (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "maintainer";

    let value = match fields.get(key)
    {
        Some(value) => value.clone(),
        None => return Ok(None)
    };
    return Ok(Some(value));
}

pub fn set_description (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "description";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };
    return Ok(Some(value.clone()));
}

pub fn set_homepage (fields: &Fields) -> Result<Option<String>, PakigeParseError>
{
    let key = "homepage";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };
    return Ok(Some(value.clone()));
}

/* ProvidesPackageList */
pub fn set_built_using (fields: &Fields) -> Result<Option<ProvidesPackageList>, PakigeParseError>
{
    let key = "built-using";
    return Ok(None);
}

pub fn set_multi_arch (fields: &Fields) -> Result<Option<MultiArch>, PakigeParseError>
{
    let key = "multi-arch";

    let value = match fields.get(key)
    {
        Some(value) => value,
        None => return Ok(None)
    };

    return match value.as_str()
    {
        // TODO: check case insensitivity
        "allowed" => Ok(Some(MultiArch::Allowed)),
        "foreign" => Ok(Some(MultiArch::Foreign)),
        "same" => Ok(Some(MultiArch::Same)),
        "no" => Ok(Some(MultiArch::No)),
        _ => Err(PakigeParseError::InvalidValue)
    };
}

// fn parse_packageref (stringdata: &str) -> PackageRef
// {
//     let pref_string = Regex::new(r"^[[:space::]]*([[:lowercase:][:digit]][[:lowercase:][:digit:][+-.]]+)[:([[:lower:][-]])]$").unwrap();
// }


