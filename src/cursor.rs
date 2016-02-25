use cmn_types::*;

pub fn offset(dir: Dir, p: &mut P)
{
    let d = to_offset(dir);

    *p = p_sum(p, &d);
}
