use cmn_types::*;

pub struct Mon
{
    p: P,
}

impl Mon
{
    pub fn new(p: &P) -> Mon
    {
        Mon { p: p.clone() }
    }

    pub fn mv(&mut self, dir: Dir)
    {
        let mut d = P { ..P::default() };

        p_offset(dir, &mut d);

        self.p = p_sum(&self.p, &d);
    }

    pub fn p(&self) -> &P
    {
        &self.p
    }
}
