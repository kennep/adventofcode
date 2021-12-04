import sys
import re

passports = []
current_passport = {}
for line in sys.stdin.readlines():
    line = line.strip()
    if current_passport and not line:
        passports.append(current_passport)
        current_passport = {}
    fields = line.split(" ")
    for field in fields:
        if ':' in field:
            key, value = field.split(":", 1)
            current_passport[key] = value
if current_passport:
    passports.append(current_passport)

required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

def valid_year(v, min, max):
    try:
        v = int(v)
        return v >= min and v <= max
    except ValueError:
        return False

def valid_height(v):
    match = re.match("^([0-9]+)((cm|in))$", v)
    if not match:
        return False
    val = int(match.group(1))
    unit = match.group(2)
    if unit == 'cm':
        return val >= 150 and val <= 193
    else:
        return val >= 59 and val <= 76

validators = {
    'byr': lambda v: valid_year(v, 1920, 2002),
    'iyr': lambda v: valid_year(v, 2010, 2020),
    'eyr': lambda v: valid_year(v, 2020, 2030),
    'hgt': lambda v: valid_height(v),
    'hcl': lambda v: re.match(r"^#[0-9a-f]{6}$", v),
    'ecl': lambda v: v in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'],
    'pid': lambda v: re.match(r"^[0-9]{9}$", v)
}

valid_passports = 0
for passport in passports:
    fields = [(k, v) for (k, v) in passport.items()
        if k in required_fields and validators.get(k, lambda v: False)(v)]
    if len(fields) >= len(required_fields):
        valid_passports += 1
        print(f"{passport}: Valid")
    else:
        print(f"{passport}: *** INVALID ***")

print(f"Valid passports: {valid_passports}")

