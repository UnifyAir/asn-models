import re
import sys

import docx

start_asn = re.compile(r"^-- ASN1START$")
end_asn = re.compile(r"^-- ASN1STOP$")

file_names = [
    "NGAP-PDU-Descriptions.asn",
    "NGAP-PDU-Contents.asn",
    "NGAP-IEs.asn",
    "NGAP-CommonDataTypes.asn",
    "NGAP-Constants.asn",
    "NGAP-Containers.asn",
]


def main(spec_file):
    doc = docx.Document(spec_file)
    start_printing = False
    out_lines = [[]]
    i = 0
    for para in doc.paragraphs:
        if re.match(start_asn, para.text):
            start_printing = True
            out_lines.append([])
            continue
        if re.match(end_asn, para.text):
            start_printing = False
            print("Stopped Printing ", i)
            i += 1
            continue
        if start_printing:
            out_lines[i].append(para.text + "\n")

    out_lines = [
        [line.replace(chr(0xA0), " ") for line in out_line] for out_line in out_lines
    ]
    preclude = [
        "--\n",
        f"-- Generated using : {' '.join(sys.argv)}\n",
        "-- DO NOT EDIT BY HAND\n",
        "--\n",
    ]
    for file_name, index in zip(file_names, range(0, i)):
        print("file_name: ", file_name)
        with open(file_name, "w") as outfile:
            outfile.writelines(out_lines[index])


if __name__ == "__main__":

    if len(sys.argv) < 4:
        print("usage: parse_spec.py <spec.docx>")
        sys.exit(1)

    spec_file = sys.argv[1]
    main(spec_file)
