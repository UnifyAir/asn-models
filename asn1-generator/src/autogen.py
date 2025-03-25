#!/usr/bin/env python3

import os
import json
import argparse
import logging

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
    logging.info(f"Processing {f}")
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
    logging.debug("---------------------Parse Tree Begin-----------------------")
    logging.debug(parse_tree.pretty())
    logging.debug("---------------------Parse Tree End-------------------------")
    all_constants = get_constants(parse_tree, constants)
    f.write(generate(parse_tree, all_constants, verbose=False, strip_xxap=xxap, config=config))
    f.close()
    logging.info(f"Generated {out}")
    result = os.system("rustfmt --edition 2024 " + out)
    if result != 0:
        logging.warning(f"rustfmt failed with exit code {result}")
    else:
        logging.debug(f"Formatted {out}")


def generate_xxap(protocol):
    constants = get_constants_from_file(
        input_file_path(protocol, input_filename(protocol, "Constants"))
    )

    logging.debug("---------------------Constants Begin-----------------------")
    logging.debug(json.dumps(constants, indent=2))
    logging.debug("---------------------Constants End-------------------------")
    # Load config.json if it exists

    # Get protocol-specific config
    protocol_config = config.get(protocol.lower(), {})
    logging.debug(f"Protocol config: {protocol_config}")

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
    logging.info(f"Completed generation for {protocol}")


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

def setup_logging(level):
    """Configure logging with the specified level."""
    numeric_level = getattr(logging, level.upper(), None)
    if not isinstance(numeric_level, int):
        raise ValueError(f"Invalid log level: {level}")
    
    logging.basicConfig(
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
        level=numeric_level
    )
    return logging.getLogger(__name__)


def create_arg_parser():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(description='Generate Rust code from ASN.1 definitions.')
    parser.add_argument('--protocol', '-p', 
                       choices=['ngap', 'f1ap', 'e1ap', 'xxap', 'rrc'],
                       nargs='+',
                       help='Protocol to generate')
    parser.add_argument('--log-level', '-l', 
                        choices=['debug', 'info', 'warning', 'error', 'critical'],
                        default='info',
                        help='Set the logging level')
    parser.add_argument('--all', '-a', action='store_true', help='Generate all protocols')
    return parser


def main():
    parser = create_arg_parser()
    args = parser.parse_args()
    logger = setup_logging(args.log_level)
    
    logger.info("Starting ASN.1 code generation")
    protocols = [] 
    if args.all:
        protocols = ["ngap", "f1ap", "e1ap", "xxap", "rrc"]
    elif args.protocol:
        protocols = args.protocol
    else:
        logger.warning("No protocol specified. Use --protocol or --all")
        parser.print_help()

    generate_xxap_common()
    for protocol in protocols:
        logger.info(f"Generating code for protocol: {protocol}")
        generate_xxap(protocol)
    
if __name__ == "__main__":
    main()
