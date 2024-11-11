// Copyright (C) 2006 Pedro Felzenszwalb
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307 USA
use std::cmp::Ordering;
use std::vec::Vec;
use std::fmt; // Import `fmt`

#[derive(Debug)]
pub struct Edge {
    pub w: f32,
    pub a: usize,
    pub b: usize,
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "w: {}, a: {}, b: {}", self.w, self.a, self)
    }
}

impl Edge {
  pub fn new(w:f32,a:usize,b:usize) -> Self {
    Edge {
        w: w,
        a: a,
        b: b,
    }
}
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.w.partial_cmp(&other.w).unwrap())
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for Edge {}

struct UniElt {
  rank: i32,
  p: usize,
  size: i32,
}

pub struct Universe {
  elts: Vec<UniElt>,
  num: usize,
}

impl Universe {
  pub fn new(elements: usize) -> Self {
      let mut elts = Vec::with_capacity(elements);
      for i in 0..elements {
          elts.push(UniElt {
              rank: 0,
              size: 1,
              p: i,
          });
      }
      Universe {
          elts,
          num: elements ,
      }
  }

  pub fn find(&mut self, x: usize) -> usize {
      let mut y = x;
      while y != self.elts[y].p {
          y = self.elts[y].p;
      }
      self.elts[x].p = y;
      y
  }

  pub fn join(&mut self, x: usize, y: usize) {
      if self.elts[x].rank > self.elts[y].rank {
          self.elts[y].p = x;
          self.elts[x].size += self.elts[y].size;
      } else {
          self.elts[x].p = y;
          self.elts[y].size += self.elts[x].size;
          if self.elts[x].rank == self.elts[y].rank {
              self.elts[y].rank += 1;
          }
      }
      self.num -= 1;
  }

  pub fn size(&self, x: usize) -> i32 {
      self.elts[x].size
  }

  pub fn num_sets(&self) -> usize {
      self.num
  }

}

