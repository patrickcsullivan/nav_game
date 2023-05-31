/// Returns `true` if there is overlap between two ranges.
///
/// If the two ranges are adjacent (i.e., `first_min == second_max` or
/// `second_min == first_max`) then they are not considered overlapping and this
/// returns `false`.
#[inline]
pub fn is_overlap(
    first_min: usize,
    first_max: usize,
    second_min: usize,
    second_max: usize,
) -> bool {
    !(first_max <= second_min || first_min >= second_max)
}

/// Given two inclusive integer ranges, returns an iterator over the integers in
/// the intersection of the two ranges.
#[inline]
pub fn intersection_incl(
    first_min: usize,
    first_max: usize,
    second_min: usize,
    second_max: usize,
) -> impl Iterator<Item = usize> {
    first_min.max(second_min)..=first_max.min(second_max)
}

/// Returns `true` if `val` is inside the exclusive range.
#[inline]
pub fn is_inside_excl(val: usize, lower: usize, upper: usize) -> bool {
    val > lower && val < upper
}
