use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut uf = Dsu::new(h * w);
    let mut grid = vec![vec![false; w]; h];

    const DIR: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {
                r: Usize1,
                c: Usize1,
            }
            grid[r][c] = true;
            for &(di, dj) in DIR.iter() {
                let nr = r + di;
                let nc = c + dj;
                if h <= nr || w <= nc || !grid[nr][nc] {
                    continue;
                }
                // (h - 1) * w + (w - 1) < h * w
                uf.merge(r * w + c, nr * w + nc);
            }
        } else {
            input! {
                r1: Usize1,
                c1: Usize1,
                r2: Usize1,
                c2: Usize1,
            }
            if grid[r1][c1] && grid[r2][c2] && uf.same(r1 * w + c1, r2 * w + c2) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

/// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/dsu.rs
/// A Disjoint set union (DSU) with union by size and path compression.
///
/// See: [Zvi Galil and Giuseppe F. Italiano, Data structures and algorithms for disjoint set union problems](https://core.ac.uk/download/pdf/161439519.pdf)
///
/// In the following documentation, let n$ be the size of the DSU.
///
/// # Example
///
/// ```
/// use ac_library_rs::Dsu;
/// use proconio::{input, source::once::OnceSource};
///
/// input! {
///     from OnceSource::from(
///         "5\n\
///          3\n\
///          0 1\n\
///          2 3\n\
///          3 4\n",
///     ),
///     n: usize,
///     abs: [(usize, usize)],
/// }
///
/// let mut dsu = Dsu::new(n);
/// for (a, b) in abs {
///     dsu.merge(a, b);
/// }
///
/// assert!(dsu.same(0, 1));
/// assert!(!dsu.same(1, 2));
/// assert!(dsu.same(2, 4));
///
/// assert_eq!(
///     dsu.groups()
///         .into_iter()
///         .map(|mut group| {
///             group.sort_unstable();
///             group
///         })
///         .collect::<Vec<_>>(),
///     [&[0, 1][..], &[2, 3, 4][..]],
/// );
/// ```
struct Dsu {
    n: usize,
    // root node: -1 * component size
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

#[allow(dead_code)]
impl Dsu {
    /// Creates a new `Dsu`.
    ///
    /// # Constraints
    ///
    /// -  \leq n \leq 10^8$
    ///
    /// # Complexity
    ///
    /// - O(n)$
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    // `	extsc` does not work in KaTeX
    /// Performs the Uɴɪᴏɴ operation.
    ///
    /// # Constraints
    ///
    /// -  \leq a < n$
    /// -  \leq b < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraints are not satisfied.
    ///
    /// # Complexity
    ///
    /// - O(\alpha(n))$ amortized
    fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    /// Returns whether the vertices a$ and b$ are in the same connected component.
    ///
    /// # Constraints
    ///
    /// -  \leq a < n$
    /// -  \leq b < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - O(\alpha(n))$ amortized
    fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    /// Performs the Fɪɴᴅ operation.
    ///
    /// # Constraints
    ///
    /// -  \leq a < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - O(\alpha(n))$ amortized
    fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    /// Returns the size of the connected component that contains the vertex a$.
    ///
    /// # Constraints
    ///
    /// -  \leq a < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - O(\alpha(n))$ amortized
    fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }

    /// Divides the graph into connected components.
    ///
    /// The result may not be ordered.
    ///
    /// # Complexity
    ///
    /// - O(n)$
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}
