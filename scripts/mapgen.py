#!/usr/bin/python

import datetime
from os import path
import re

def variant(num):
    if num == 1:
        return 'Fold::One'
    elif num == 2:
        return 'Fold::Two'
    elif num == 3:
        return 'Fold::Three'
        
txt = open('./scripts/CaseFolding.txt')

def replacement(chars):
    chars_len = len(chars)
    inside = ', '.join(["'\\u{%04x}'" % c for c in chars])
    return '%s(%s)' % (variant(chars_len), inside)

def apply_constant_offset(offset_from, offset_to):
    if offset_to < offset_from:
        return "wrapping_sub(0x%04x)" % (offset_from - offset_to)
    else:
        return "wrapping_add(0x%04x)" % (offset_to - offset_from)

rs = open(path.abspath('./src/unicode/map.rs'), 'w')

run_in_progress = None

class Run:

    def __init__(self, map_from, map_tos):
        self.start = map_from
        self.end = map_from
        self.map_tos = map_tos
        self.every_other = None
        
    def limit_to_range(self, min_relevant, max_relevant):
        
        if self.end < min_relevant: return None
        if self.start > max_relevant: return None
        
        if self.start >= min_relevant and self.end <= max_relevant: return self
        
        ret = Run(self.start, [m for m in self.map_tos])
        ret.end = self.end
        ret.every_other = self.every_other
        if ret.start < min_relevant:
            diff = min_relevant - ret.start
            if ret.every_other == True and diff%2==1:
                diff += 1
            ret.start += diff
            ret.map_tos[0] += diff
        if ret.end > max_relevant:
            ret.end = max_relevant
        
        return ret
        
    def expand_into(self, map_from, map_tos):
        if len(self.map_tos) != 1 or len(map_tos) != 1:
            # Do not attempt to combine if we are not mapping to one character. Those do not follow a simple pattern.
            return False
            
        if self.every_other!=True and self.end + 1 == map_from and map_tos[0] == self.map_tos[0] + (map_from - self.start):
            self.end += 1
            self.every_other = False
            return True
        if self.every_other!=False and self.end + 2 == map_from and map_tos[0] == self.map_tos[0] + (map_from - self.start):
            self.end += 2
            self.every_other = True
            return True
            
        return False

    # When dumping ranges, we avoid using range literals to maintain compatibility with old rustcs
    def dump(self, match_on_low_byte = False):
        def format_range_edge(x):
            if match_on_low_byte:
                return '0x%02x' % (x&0xff)
            else:
                return '0x%04x' % x
        def remove_useless_comparison(case_line):
            case_line = case_line.replace("0x00 <= x && ", "")
            if match_on_low_byte:
                case_line = case_line.replace(" && x <= 0xff", "")
            return case_line

        if self.start == self.end:
            if len(self.map_tos)==1:
                rs.write("            %s => 0x%04x,\n" % (format_range_edge(self.start), self.map_tos[0]))
            else:
                rs.write("            %s => return %s,\n" % (format_range_edge(self.start), replacement(self.map_tos)))
        elif self.every_other != True:
            rs.write(remove_useless_comparison("            x if (%s..=%s).contains(&x) => from.%s,\n" % (format_range_edge(self.start), format_range_edge(self.end), apply_constant_offset(self.start, self.map_tos[0]))),)
        elif self.map_tos[0] - self.start == 1 and self.start%2==0:
            rs.write(remove_useless_comparison("            x if (%s..=%s).contains(&x) => from | 1,\n" % (format_range_edge(self.start), format_range_edge(self.end))))
        elif self.map_tos[0] - self.start == 1 and self.start%2==1:
            rs.write(remove_useless_comparison("            x if (%s..=%s).contains(&x) => (from + 1) & !1,\n" % (format_range_edge(self.start), format_range_edge(self.end))))
        else:
            rs.write("            x if (%s..=%s).contains(&x) => {\n" % (format_range_edge(self.start), format_range_edge(self.end)))
            rs.write("                    if (from & 1) == %s {\n" % (self.start % 2))
            rs.write("                        from.%s\n" % (apply_constant_offset(self.start, self.map_tos[0])))
            rs.write("                    } else {\n")
            rs.write("                        from\n")
            rs.write("                    }\n")
            rs.write("                }\n")

runs = []
singlet_runs = [] # for test generation

lines = txt.readlines()
first_line_match = re.match(r"# \w+-(.+)\.txt", lines[0])
unicode_version = first_line_match[1]

for line in lines:
    if line[0] != '#':
        parts = line.split('; ')
        if len(parts) > 2 and parts[1] in 'CF':
            map_from = int(parts[0], 16)
            map_tos = [int(char, 16) for char in parts[2].split(' ')]
            
            if run_in_progress and run_in_progress.expand_into(map_from, map_tos):
                pass
            else:
                if run_in_progress: runs.append(run_in_progress)
                run_in_progress = Run(map_from, map_tos)
            singlet_runs.append(Run(map_from, map_tos))
runs.append(run_in_progress)

high_runs = [r for r in runs if r.end > 0x2CFF]
    
small_run_chunks = [] # Each element of this corresponds to a high byte being mapped from
for high_byte in range(0, 0x2D):
    minimum_relevant = (high_byte<<8)
    maximum_relevant = minimum_relevant + 255
    run_chunk = []
    for run in runs:
        subrun = run.limit_to_range(minimum_relevant, maximum_relevant)
        if subrun:
            run_chunk.append(subrun)
    small_run_chunks.append(run_chunk)

rs.write('// Case-folding function for Unicode %s\n' % unicode_version)
rs.write('// Generated by scripts/mapgen.py\n')
rs.write('// %s\n' % datetime.date.today())
rs.write('\n')
rs.write('use super::fold::Fold;\n\n')
rs.write('use core::char;\n');
rs.write("pub fn lookup(orig: char) -> Fold {\n")
rs.write('    // The code below is is intended to reduce the binary size from that of a simple 1:1 lookup table.\n')
rs.write('    // It exploits two facts:\n')
rs.write('    // 1. Many of the mappings form ranges mapped to other ranges.\n')
rs.write('    //    To benefit from this, we match on ranges instead of single numbers.\n')
rs.write('    //    Alone, this decreases the binary size but results in performance regression over the simple 1:1 lookup.\n');
rs.write('    // 2. Most of the mappings are from relatively small chars (0 - 0x2CFF).\n')
rs.write('    //    To benefit from this, we use a jump table based on the high byte for this range.\n')
rs.write('    //    This more than recovers the performance regression from exploting fact #1, at least in the tested benchmark.\n')
rs.write('    let from = orig as u32;\n')
rs.write('    if from <= 0x2CFF {\n')
rs.write('        let from = from as u16;\n');
rs.write('        let high_byte = (from >> 8) as u8;\n');
rs.write('        let low_byte = (from & 0xff) as u8;\n');
rs.write('        let single_char: u16 = match high_byte {\n')
for (high_byte, runs) in enumerate(small_run_chunks):
    rs.write("            0x%02x => " % high_byte);
    if len(runs)==0:
        rs.write('from,\n')
    else:
        rs.write("match low_byte {\n")
        for r in runs:
            rs.write('    ')
            r.dump(match_on_low_byte = True)
        rs.write("                _ => from,\n")
        rs.write("            },\n");
rs.write('            _ => from,\n')
rs.write('        };\n');
rs.write('        Fold::One(char::from_u32(single_char as u32).unwrap_or(orig))\n')
rs.write('    } else {\n');
rs.write('        let single_char: u32 = match from {\n')
for r in high_runs:
    r.dump()
rs.write('            _ => from,\n')
rs.write('        };\n')
rs.write('        Fold::One(char::from_u32(single_char).unwrap_or(orig))\n')
rs.write('    }\n')
rs.write('}\n')


test_max = singlet_runs[-1].end + 1000

rs.write('\n');
rs.write('#[test]\n');
rs.write('fn lookup_consistency() {\n');
rs.write('    fn lookup_naive(orig: char) -> Fold {\n')
rs.write('        let single_char = match orig as u32 {\n');
for r in singlet_runs:
    r.dump()
rs.write('            _ => orig as u32,\n')
rs.write('        };\n')
rs.write('        Fold::One(char::from_u32(single_char).unwrap())\n')
rs.write('    }\n\n')
rs.write('    for c_index in 0..%d {\n' % test_max)
rs.write('        if let Some(c) = char::from_u32(c_index) {\n');
rs.write('            let reference: Vec<char> = lookup_naive(c).collect();\n')
rs.write('            let actual: Vec<char> = lookup(c).collect();\n')
rs.write('            if actual != reference {\n')
rs.write('                assert!(\n')
rs.write('                    false,\n')
rs.write('                    "case-folding {:?} (#0x{:04x}) failed: Expected {:?}, got {:?}",\n')
rs.write('                    c, c_index, reference, actual,\n')
rs.write('                );\n')
rs.write('            }\n')
rs.write('        }\n')
rs.write('    }\n')
rs.write('}\n');
