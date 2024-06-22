fn find_palindrome(v: &Vec<i32>) -> usize {
    let mut start = 0;
    let mut end = 1;
    let mut l = 0;
    let mut r = 0;
    for i in 0..v.len() as i32 {
        l = i - 1;
        r = i;
        while l >= 0 && r < v.len() as i32 && v[l as usize] == v[r as usize] {
            if r - l + 1 > end {
                start = l;
                end = r - l + 1;
            }
            l -= 1;
            r += 1;
        }

        l = i - 1;
        r = i + 1;

        while l >= 0 && r < v.len() as i32 && v[l as usize] == v[r as usize] {
            if r - l + 1 > end {
                start = l;
                end = r - l + 1;
            }
            l -= 1;
            r += 1;
        }
    }
    return start as usize;
}
