use std::collections::HashMap;
use crate::PakigeParseError;
use super::{BinaryDeb, Fields, MultiArch};
use regex::Regex;

type SetterFn = fn(BinaryDeb) -> Result<BinaryDeb, PakigeParseError>;

pub static BINARYDEB_SETTERS: [SetterFn; 22] = 
[
    binarydeb_package,
    binarydeb_source,
    binarydeb_version,
    binarydeb_section,
    binarydeb_priority,
    binarydeb_architecture,
    binarydeb_essential,
    binarydeb_depends,
    binarydeb_recommends,
    binarydeb_suggests,
    binarydeb_enhances,
    binarydeb_pre_depends,
    binarydeb_breaks,
    binarydeb_conflicts,
    binarydeb_provides,
    binarydeb_replaces,
    binarydeb_installed_size,
    binarydeb_maintainer,
    binarydeb_description,
    binarydeb_homepage,
    binarydeb_built_using,
    binarydeb_multi_arch,
];

/* Mandatory */
fn binarydeb_package (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "package";

    let mut value = deb.all_fields.get(key)
    .ok_or(PakigeParseError::MissingMandatoryField)?;

    let name_rules = Regex::new(r"[[:lowercase:][:digit]][[:lowercase:][:digit:][+-.]]+").unwrap();
    if name_rules.is_match(value)
    {
        deb.package = value.clone();
        return Ok(deb);
    }
    return Err(PakigeParseError::InvalidValue);
}

fn binarydeb_source (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "source";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            let name_rules = Regex::new(r"[[:lowercase:][:digit]][[:lowercase:][:digit:][+-.]]+").unwrap();
            if name_rules.is_match(value)
            {
                deb.source = Some(value.clone());
                Ok(deb)
            }
            else
            {
                Err(PakigeParseError::InvalidValue)
            }
        },
        None => Ok(deb) /* no error, not mandatory */
    };
}

/* Mandatory */ //TODO: create structs, validate syntax of version string
fn binarydeb_version (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "version";
    let mut value = deb.all_fields.get(key)
    .ok_or(PakigeParseError::MissingMandatoryField)?;

    deb.version = value.clone();
    return Ok(deb);
}

// TODO: reflect the official list of sections
fn binarydeb_section (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "section";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.section = Some(value.clone());
            Ok(deb)
        },
        None => Ok(deb) /* no error, not mandatory */
    };
}

// TODO: reflect the official list of priorities
fn binarydeb_priority (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "priority";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.priority = value.clone();
            Ok(deb)
        },
        None =>
        {
            deb.priority = String::from("optional");
            Ok(deb) /* no error, has default value */
        }
    };
}

/* Mandatory */ // TODO: reflect the official list of architectures, or at least syntax
fn binarydeb_architecture (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "architecture";
    let mut value = deb.all_fields.get(key)
    .ok_or(PakigeParseError::MissingMandatoryField)?;

    deb.architecture = value.clone();
    return Ok(deb);
}

fn binarydeb_essential (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "essential";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.essential = match value.as_str()
            {
                "yes" => true,
                "no" => false,
                _ => return Err(PakigeParseError::InvalidValue)
            };
            Ok(deb)
        },
        None =>
        {
            Ok(deb) /* no error, has default value */
        }
    };
}

/* DependsPackageList */
fn binarydeb_depends (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "depends";
    return Ok(deb);
}

/* DependsPackageList */
fn binarydeb_recommends (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "recommends";
    return Ok(deb);
}

/* DependsPackageList */
fn binarydeb_suggests (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "suggests";
    return Ok(deb);
}

/* DependsPackageList */
fn binarydeb_enhances (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "enhances";
    return Ok(deb);
}

/* DependsPackageList */
fn binarydeb_pre_depends (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "pre-depends";
    return Ok(deb);
}

/* ProvidesPackageList */
fn binarydeb_breaks (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "breaks";
    return Ok(deb);
}

/* ProvidesPackageList */
fn binarydeb_conflicts (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "conflicts";
    return Ok(deb);
}

/* ProvidesPackageList */
fn binarydeb_provides (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "provides";
    return Ok(deb);
}

/* ProvidesPackageList */
fn binarydeb_replaces (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "replaces";
    return Ok(deb);
}

fn binarydeb_installed_size (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "installed-size";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.installed_size = match value.parse::<u64>()
            {
                Ok(size) => Some(size),
                Err(e) => return Err(PakigeParseError::InvalidValue)
            };
            Ok(deb)
        },
        None => Ok(deb) /* no error, not mandatory */
    };
}

/* Mandatory */
fn binarydeb_maintainer (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "maintainer";
    let mut value = deb.all_fields.get(key)
    .ok_or(PakigeParseError::MissingMandatoryField)?;

    deb.maintainer = value.clone();
    return Ok(deb);
}

/* Mandatory */
fn binarydeb_description (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "description";
    let mut value = deb.all_fields.get(key)
    .ok_or(PakigeParseError::MissingMandatoryField)?;

    deb.description = value.clone();
    return Ok(deb);
}

fn binarydeb_homepage (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "homepage";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.homepage = Some(value.clone());
            Ok(deb)
        },
        None => Ok(deb) /* no error, not mandatory */
    };
}

fn binarydeb_built_using (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "built-using";
    return Ok(deb);
}

fn binarydeb_multi_arch (mut deb: BinaryDeb) -> Result<BinaryDeb, PakigeParseError>
{
    let key = "multi-arch";
    return match deb.all_fields.get(key)
    {
        Some(value) =>
        {
            deb.multi_arch = match value.as_str()
            {
                "allowed" => MultiArch::Allowed,
                "foreign" => MultiArch::Foreign,
                "same" => MultiArch::Same,
                "no" => MultiArch::No,
                _ => return Err(PakigeParseError::InvalidValue)
            };
            Ok(deb)
        },
        None => Ok(deb) /* no error, has default value */
    };
}
