pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut iter = height.iter().enumerate();
    let mut p1 = iter.next();
    let mut p2 = iter.next_back();
    while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
        let area = (j - i) as i32 * h1.min(h2);
        max_area = max_area.max(area);
        if h1 < h2 {
            p1 = iter.next();
        } else {
            p2 = iter.next_back();
        }
    }
    max_area
}