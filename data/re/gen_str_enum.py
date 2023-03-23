import json;
import re;

names = set()

def norm_name(v):
    if len(v) == 0:
        return "empty"
    
    if v[0].isdigit():
        v = '_' + v
    v = v.replace('%', 'FMT_')
    return re.sub(r'\W+', '', v).strip()[0:20]


def get_name(v):
    v = v.strip()
    if "/" in v:
        return norm_name(v.split('/')[-1])
    
    return norm_name(v)


def uniq_name(v):
    if v in names:
        u = v
        for i in range(1, 1000):
            v = f'{u}_{i}'
            if not v in names:
                break

    names.add(v)
    return v

opts = list()
with open('data/re/strs.json') as f:
    data = json.load(f)
    for entry in list(data):
        id = entry['i']
        val = entry['str']
        if id >= 10_000:
            continue
        v = uniq_name(get_name(val))
        opts.append((id, v))

opts.sort(key= lambda v: v[0])

with open('str_enum.h', 'w') as f:
    f.write('enum StrPoolCodes {\n')
    for v in opts:
        f.write(f'\t{v[1]} = {v[0]},\n')
    f.write('}')