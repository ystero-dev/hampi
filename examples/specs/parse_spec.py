import sys
import re

import docx

start_asn = re.compile(r'^-- ASN1START$')
end_asn = re.compile(r'^-- ASN1STOP$')

def main(spec_file, asn_file):

    doc = docx.Document(spec_file)
    start_printing = False
    out_lines = []
    for para in doc.paragraphs:
        if re.match(start_asn, para.text):
            start_printing = True
        if start_printing:
            out_lines.append(para.text + "\n")
        if re.match(end_asn, para.text):
            start_printing = False

    out_lines = [line.replace(chr(0xa0), ' ') for line in out_lines]
    preclude = [
            "--\n",
            f"-- Generated using : {' '.join(sys.argv)}\n",
            "-- DO NOT EDIT BY HAND\n",
            "--\n"]
    with open(asn_file, 'w') as outfile:
        outfile.writelines(preclude)
        outfile.writelines(out_lines)


if __name__ == '__main__':

    if len(sys.argv) < 4:
        print("usage: parse_spec.py <spec.docx> -o <out.asn>")
        sys.exit(1)

    spec_file = sys.argv[1]
    asn_file = sys.argv[3]
    main(spec_file, asn_file)
