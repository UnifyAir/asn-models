#!/usr/bin/env python3

import os

from constants import get_constants, get_constants_from_file
from parse import parse_file
from render import generate
from config import config

AUTOGENERATED = "// Autogenerated from "
CLIPPY = "#![allow(clippy::all)]"
USE_COMMON = """\
use super::common::*;
#[allow(unused_imports)]
use xxap::{GtpTunnel, TransportLayerAddress, PduSessionId, GtpTeid};
"""
USE_ASN1_APER = """\
use asn1_per::{{aper::*, *}};
"""
USE_ASN1_UPER = """\
use asn1_per::{{uper::*, *}};
"""
USE_IES = "use super::ies::*;\n"
USE_PDUS = """\
use crate::common::Criticality;
use super::pdu::*;
use async_trait::async_trait;
use anyhow::Result;
"""
USE_RRC_SETUP_RELEASE = """\
use crate::SetupRelease;
"""

def output_filename(protocol, filename):
    return f"../../{protocol.lower()}/src/{filename}"


def input_filename(protocol, filename_part):
    f = f"{protocol.upper()}-{filename_part}.asn"
    print("Processing " + f)
    return f


def input_file_path(protocol, filename):
    return f"../asn1/{protocol.lower()}/{filename}"


def generate_file(protocol, outfile, infile, header, constants, xxap, parse_tree=None, config={}):
    i = input_filename(protocol, infile)
    out = output_filename(protocol, outfile)
    f = open(out, "w")
    f.write(AUTOGENERATED + i + "\n" + CLIPPY + "\n")
    f.write(header + "\n\n")
    parse_tree = parse_tree or parse_file(input_file_path(protocol, i))
    all_constants = get_constants(parse_tree, constants)
    f.write(generate(parse_tree, all_constants, verbose=False, strip_xxap=xxap, config=config))
    f.close()
    os.system("rustfmt --edition 2024 " + out)


def generate_xxap(protocol):
    constants = get_constants_from_file(
        input_file_path(protocol, input_filename(protocol, "Constants"))
    )
    
    # Load config.json if it exists

    # Get protocol-specific config
    protocol_config = config.get(protocol.lower(), {})

    generate_file(
        protocol,
        "top_pdu.rs",
        "PDU-Descriptions",
        USE_ASN1_APER + USE_PDUS,
        constants,
        True,
        config=protocol_config,
    )
    generate_file(
        protocol, 
        "common.rs", 
        "CommonDataTypes", 
        USE_ASN1_APER, 
        constants, 
        True,
        config=protocol_config,
    )
    generate_file(
        protocol, "ies.rs", "IEs", USE_COMMON + USE_ASN1_APER, constants, True, config=protocol_config
    )
    generate_file(
        protocol,
        "pdu.rs",
        "PDU-Contents",
        USE_COMMON + USE_ASN1_APER + USE_IES,
        constants,
        True,
        config=protocol_config,
    )


def generate_xxap_common():
    generate_file("xxap", "ies.rs", "Common-IEs", USE_ASN1_APER, dict(), False)


def generate_rrc():
    protocol = "rrc"
    parse_tree = parse_file(input_file_path(protocol, input_filename(protocol, "All")))
    constants = get_constants(parse_tree)

    generate_file(
        protocol,
        "rrc.rs",
        "All",
        USE_ASN1_UPER + USE_RRC_SETUP_RELEASE,
        constants,
        False,
        parse_tree,
    )


generate_xxap_common()
generate_xxap("ngap")
