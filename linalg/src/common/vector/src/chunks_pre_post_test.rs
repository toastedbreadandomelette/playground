#[cfg(test)]
pub mod tests {
    use super::super::vector::Vector;

    #[test]
    pub fn window_test() {
        let x: Vector<u32> = (1..=23).collect();
        let y: Vector<u32> = (1..=46).collect();
        x.chunks_with_pre_and_post_mut(5)
            .zip(y.chunks_with_pre_and_post_mut(10))
            .enumerate()
            .for_each(|(index, ((pre, curr, post), (p0, p1, p2)))| {
                // Make current value: curr[index] to be
            });
    }
}
